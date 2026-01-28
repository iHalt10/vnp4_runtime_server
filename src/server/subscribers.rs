use crate::server::subscriber::Subscriber;
use crate::utils::p4runtime::as_u128_from;
use crate::utils::p4runtime::as_uint128_from;
use p4runtime::google::rpc::Code;
use p4runtime::google::rpc::Status;
use p4runtime::p4::v1::MasterArbitrationUpdate;
use p4runtime::p4::v1::StreamMessageResponse;
use p4runtime::p4::v1::Uint128;
use p4runtime::p4::v1::stream_message_response::Update;
use std::collections::HashMap;
use tracing::info;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct Subscribers {
    device_id: u64,
    subscribers: HashMap<u128, Subscriber>,
}

impl Subscribers {
    pub fn new(device_id: u64) -> Self {
        Self {
            device_id: device_id,
            subscribers: HashMap::new(),
        }
    }

    pub fn insert(&mut self, subscriber: &Subscriber) {
        let old_primary_id = self.get_primary_election_id();
        let election_id = as_u128_from(&subscriber.election_id);
        self.subscribers.insert(election_id, subscriber.clone());
        let new_primary_id = self.get_primary_election_id();
        if old_primary_id == new_primary_id {
            self.notify_one(&subscriber.clone(), new_primary_id.unwrap());
        } else {
            self.notify_all(new_primary_id.unwrap());
        }
    }

    pub fn delete(&mut self, election_id: Uint128) {
        let election_id: u128 = as_u128_from(&election_id);
        let removed = self.subscribers.remove(&election_id).is_some();
        if removed {
            let new_primary_id = self.get_primary_election_id();
            if new_primary_id.is_some() {
                self.notify_all(new_primary_id.unwrap());
            }
        }
    }

    fn get_primary_election_id(&self) -> Option<u128> {
        self.subscribers.keys().max().copied()
    }

    pub fn find(&self, election_id: Uint128) -> Option<&Subscriber> {
        let election_id: u128 = as_u128_from(&election_id);
        self.subscribers.get(&election_id)
    }
    pub fn find_mut(&mut self, election_id: Uint128) -> Option<&mut Subscriber> {
        let election_id: u128 = as_u128_from(&election_id);
        self.subscribers.get_mut(&election_id)
    }

    pub fn get_primary(&self) -> Option<&Subscriber> {
        self.subscribers.values().max_by_key(|subscriber| as_u128_from(&subscriber.election_id))
    }

    pub fn len(&self) -> usize {
        self.subscribers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.subscribers.is_empty()
    }

    pub fn clear(&mut self) {
        self.subscribers.clear();
    }

    fn notify_all(&self, primary_election_id: u128) {
        for subscriber in self.subscribers.values() {
            self.notify_one(subscriber, primary_election_id);
        }
    }

    fn notify_one(&self, subscriber: &Subscriber, primary_election_id: u128) {
        let election_id = as_u128_from(&subscriber.election_id);
        let status = if election_id == primary_election_id {
            Some(Status {
                code: Code::Ok.into(),
                message: "Primary client".to_string(),
                details: Vec::new(),
            })
        } else {
            Some(Status {
                code: Code::AlreadyExists.into(),
                message: "Not primary client".to_string(),
                details: Vec::new(),
            })
        };

        let update = MasterArbitrationUpdate {
            device_id: self.device_id,
            role: None,
            election_id: Some(as_uint128_from(primary_election_id).clone()),
            status,
        };

        let response = StreamMessageResponse {
            update: Some(Update::Arbitration(update)),
        };

        match subscriber.sender.try_send(Ok(response)) {
            Ok(_) => {
                info!(
                    election_id = %election_id,
                    device_id = %self.device_id,
                    "Message sent to subscriber"
                );
            }
            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => {
                warn!(
                    election_id = %election_id,
                    device_id = %self.device_id,
                    "Channel full for subscriber, message dropped"
                );
            }
            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
                warn!(
                    election_id = %election_id,
                    device_id = %self.device_id,
                    "Channel closed for subscriber"
                );
            }
        }
    }
}

impl IntoIterator for Subscribers {
    type Item = (u128, Subscriber);
    type IntoIter = std::collections::hash_map::IntoIter<u128, Subscriber>;

    fn into_iter(self) -> Self::IntoIter {
        self.subscribers.into_iter()
    }
}

impl<'a> IntoIterator for &'a Subscribers {
    type Item = (&'a u128, &'a Subscriber);
    type IntoIter = std::collections::hash_map::Iter<'a, u128, Subscriber>;

    fn into_iter(self) -> Self::IntoIter {
        self.subscribers.iter()
    }
}
