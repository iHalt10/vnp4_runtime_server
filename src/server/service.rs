use crate::server::config::ServerConfig;
use crate::server::connection::Connection;
use crate::server::subscriber::Subscriber;
use crate::target::models::Device;
use crate::utils::p4runtime::get_forwarding_pipeline_config_request::as_response_type;
use p4runtime::p4::v1::CapabilitiesRequest;
use p4runtime::p4::v1::CapabilitiesResponse;
use p4runtime::p4::v1::Entity;
use p4runtime::p4::v1::ForwardingPipelineConfig;
use p4runtime::p4::v1::GetForwardingPipelineConfigRequest;
use p4runtime::p4::v1::GetForwardingPipelineConfigResponse;
use p4runtime::p4::v1::ReadRequest;
use p4runtime::p4::v1::ReadResponse;
use p4runtime::p4::v1::SetForwardingPipelineConfigRequest;
use p4runtime::p4::v1::SetForwardingPipelineConfigResponse;
use p4runtime::p4::v1::StreamMessageRequest;
use p4runtime::p4::v1::StreamMessageResponse;
use p4runtime::p4::v1::Uint128;
use p4runtime::p4::v1::WriteRequest;
use p4runtime::p4::v1::WriteResponse;
use p4runtime::p4::v1::get_forwarding_pipeline_config_request::ResponseType;
use p4runtime::p4::v1::p4_runtime_server::P4Runtime;
use p4runtime::p4::v1::p4_runtime_server::P4RuntimeServer;
use p4runtime::p4::v1::stream_message_request::Update as StreamMessageRequestUpdate;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::Streaming;
use tonic::transport::Server;
use tracing::error;
use tracing::info;
use tracing::warn;

#[derive(Debug)]
pub struct P4RuntimeService {
    config: Arc<ServerConfig>,
    devices: Arc<HashMap<u64, Device>>,
}

impl P4RuntimeService {
    pub fn new(config: ServerConfig, devices: HashMap<u64, Device>) -> Self {
        Self {
            config: Arc::new(config),
            devices: Arc::new(devices),
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = format!("{}:{}", self.config.address, self.config.port).parse()?;

        info!("Starting gRPC server on {}", endpoint);

        Server::builder().add_service(P4RuntimeServer::new(self)).serve(endpoint).await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl P4Runtime for P4RuntimeService {
    type StreamChannelStream = Pin<Box<dyn Stream<Item = Result<StreamMessageResponse, Status>> + Send>>;
    type ReadStream = Pin<Box<dyn Stream<Item = Result<ReadResponse, Status>> + Send>>;

    async fn write(&self, request: Request<WriteRequest>) -> Result<Response<WriteResponse>, Status> {
        let req = request.into_inner();

        let device_id = req.device_id;
        let device = self.devices.get(&device_id).ok_or_else(|| Status::not_found(format!("Device {} not found", device_id)))?;
        for update in req.updates {
            let update_type = p4runtime::p4::v1::update::Type::try_from(update.r#type).map_err(|_| Status::invalid_argument("Invalid update type"))?;

            match update.entity.and_then(|e| e.entity) {
                Some(p4runtime::p4::v1::entity::Entity::TableEntry(table_entry)) => {
                    let table_id = table_entry.table_id;
                    let table_lock = device.tables.get(&table_id).ok_or_else(|| Status::not_found(format!("Table {} not found", table_id)))?;
                    let mut table = table_lock.write().await;
                    if let Err(e) = table.apply(table_entry, update_type) {
                        error!(
                            device_id = %device_id,
                            table_id = %table_id,
                            error = %e,
                            "Failed to apply table entry"
                        );
                        return Err(Status::internal(format!("Failed to apply table entry: {}", e)));
                    }
                }
                _ => {
                    return Err(Status::unimplemented("Entity type not supported"));
                }
            }
        }

        let response = WriteResponse::default();
        Ok(Response::new(response))
    }

    async fn read(&self, request: Request<ReadRequest>) -> Result<Response<Self::ReadStream>, Status> {
        let req = request.into_inner();

        let device_id = req.device_id;
        let device = self.devices.get(&device_id).ok_or_else(|| Status::not_found(format!("Device {} not found", device_id)))?;

        let (tx, rx) = mpsc::channel(100);
        let entities = req.entities;
        let tables = Arc::clone(&device.tables);

        tokio::spawn(async move {
            for entity in entities {
                let response_entities = match entity.entity {
                    Some(p4runtime::p4::v1::entity::Entity::TableEntry(table_entry)) => {
                        let table_id = table_entry.table_id;
                        let table_lock = match tables.get(&table_id) {
                            Some(lock) => lock,
                            None => {
                                let _ = tx.send(Err(Status::not_found(format!("Table {} not found", table_id)))).await;
                                return;
                            }
                        };
                        let table = table_lock.read().await;
                        table
                            .entries
                            .values()
                            .cloned()
                            .map(|table_entry| Entity {
                                entity: Some(p4runtime::p4::v1::entity::Entity::TableEntry(table_entry)),
                            })
                            .collect()
                    }
                    _ => {
                        vec![]
                    }
                };

                for resp_entity in response_entities {
                    let read_response = ReadResponse { entities: vec![resp_entity] };

                    if tx.send(Ok(read_response)).await.is_err() {
                        warn!("Client disconnected during read");
                        return;
                    }
                }
            }

            info!("Read request processing completed");
        });

        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }

    async fn set_forwarding_pipeline_config(&self, _request: Request<SetForwardingPipelineConfigRequest>) -> Result<Response<SetForwardingPipelineConfigResponse>, Status> {
        let response = SetForwardingPipelineConfigResponse {};
        Ok(Response::new(response))
    }

    async fn get_forwarding_pipeline_config(&self, request: Request<GetForwardingPipelineConfigRequest>) -> Result<Response<GetForwardingPipelineConfigResponse>, Status> {
        let req = request.into_inner();
        let device_id = req.device_id;
        let response_type: ResponseType = as_response_type(req.response_type).unwrap();
        let device = self.devices.get(&device_id).ok_or_else(|| Status::not_found(format!("Device {} not found", device_id)))?;

        match response_type {
            ResponseType::All => {
                let config = ForwardingPipelineConfig {
                    p4info: Some(device.as_p4info()),
                    p4_device_config: Vec::new(),
                    cookie: None,
                };
                let response = GetForwardingPipelineConfigResponse { config: Some(config) };
                return Ok(Response::new(response));
            }
            ResponseType::CookieOnly => {
                let config = ForwardingPipelineConfig {
                    p4info: None,
                    p4_device_config: Vec::new(),
                    cookie: None,
                };
                let response = GetForwardingPipelineConfigResponse { config: Some(config) };
                return Ok(Response::new(response));
            }
            ResponseType::P4infoAndCookie => {
                let config = ForwardingPipelineConfig {
                    p4info: Some(device.as_p4info()),
                    p4_device_config: Vec::new(),
                    cookie: None,
                };
                let response = GetForwardingPipelineConfigResponse { config: Some(config) };
                return Ok(Response::new(response));
            }
            ResponseType::DeviceConfigAndCookie => {
                let config = ForwardingPipelineConfig {
                    p4info: None,
                    p4_device_config: Vec::new(),
                    cookie: None,
                };
                let response = GetForwardingPipelineConfigResponse { config: Some(config) };
                return Ok(Response::new(response));
            }
        }
    }

    async fn stream_channel(&self, request: Request<Streaming<StreamMessageRequest>>) -> Result<Response<Self::StreamChannelStream>, Status> {
        let mut inbound = request.into_inner();
        let (tx, rx) = mpsc::channel::<Result<StreamMessageResponse, Status>>(100);
        let responder = tx.clone();

        let devices = self.devices.clone();
        let mut connection = Connection::new(self.devices.as_ref());

        tokio::spawn(async move {
            while let Some(result) = inbound.next().await {
                match result {
                    Ok(message) => match message.update.unwrap() {
                        StreamMessageRequestUpdate::Arbitration(arbitration) => {
                            let device = devices.get(&arbitration.device_id);
                            if device.is_none() {
                                continue;
                            }
                            let device: &Device = device.unwrap();
                            if connection.subscribed(arbitration.device_id) {
                                let old_election_id = connection.get_election_id(arbitration.device_id).unwrap();
                                let new_election_id = arbitration.election_id.unwrap_or(Uint128 { high: 0, low: 0 });
                                let mut subscribers = device.subscribers.write().await;
                                if subscribers.find_mut(new_election_id).is_some() {
                                    continue;
                                }
                                subscribers.delete(old_election_id);
                                let new_subscriber = Subscriber::new(new_election_id, tx.clone());
                                subscribers.insert(&new_subscriber);
                                connection.resubscribe(arbitration.device_id, new_election_id);
                            } else {
                                let election_id = arbitration.election_id.unwrap_or(Uint128 { high: 0, low: 0 });
                                let mut subscribers = device.subscribers.write().await;
                                if subscribers.find(election_id).is_some() {
                                    continue;
                                }
                                let subscriber = Subscriber::new(election_id, tx.clone());
                                subscribers.insert(&subscriber);
                                connection.subscribe(arbitration.device_id, election_id);
                            }
                        }
                        _ => {}
                    },
                    Err(e) => {
                        let _ = responder.send(Err(e)).await;
                        break;
                    }
                }
            }
            for (device_id, election_id) in connection.get_subscribed_list().iter() {
                let device = devices.get(&device_id).unwrap();

                let mut subscribers = device.subscribers.write().await;
                subscribers.delete(*election_id);
            }
            info!("Client stream channel processing ended");
        });
        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }

    async fn capabilities(&self, _request: Request<CapabilitiesRequest>) -> Result<Response<CapabilitiesResponse>, Status> {
        let response = CapabilitiesResponse {
            p4runtime_api_version: "v1".to_string(),
            experimental: Default::default(),
        };
        Ok(Response::new(response))
    }
}
