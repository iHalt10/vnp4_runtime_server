use p4runtime::p4::v1::StreamMessageResponse;
use p4runtime::p4::v1::Uint128;
use tokio::sync::mpsc::Sender;
use tonic::Status;

#[derive(Debug, Clone)]
pub struct Subscriber {
    pub election_id: Uint128,
    pub sender: Sender<Result<StreamMessageResponse, Status>>,
}

impl Subscriber {
    pub fn new(election_id: Uint128, sender: Sender<Result<StreamMessageResponse, Status>>) -> Self {
        Self {
            election_id: election_id,
            sender: sender,
        }
    }
}
