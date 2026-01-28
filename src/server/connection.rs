use crate::target::models::Device;
use p4runtime::p4::v1::Uint128;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Connection {
    status: HashMap<u64, (Option<Uint128>, bool)>,
}

impl Connection {
    pub fn new(devices: &HashMap<u64, Device>) -> Self {
        let mut status: HashMap<u64, (Option<Uint128>, bool)> = HashMap::new();
        for id in devices.keys() {
            status.insert(*id, (None, false));
        }
        Self { status: status }
    }

    pub fn get_election_id(&self, device_id: u64) -> Option<Uint128> {
        self.status.get(&device_id).unwrap().0
    }

    pub fn subscribed(&self, device_id: u64) -> bool {
        self.status.get(&device_id).unwrap().1
    }

    pub fn subscribe(&mut self, device_id: u64, election_id: Uint128) {
        *self.status.get_mut(&device_id).unwrap() = (Some(election_id), true);
    }

    pub fn resubscribe(&mut self, device_id: u64, election_id: Uint128) {
        self.subscribe(device_id, election_id);
    }

    pub fn unsubscribe(&mut self, device_id: u64) {
        *self.status.get_mut(&device_id).unwrap() = (None, false);
    }

    pub fn get_subscribed_list(&self) -> HashMap<u64, Uint128> {
        let mut subscribed_list: HashMap<u64, Uint128> = HashMap::new();
        for (device_id, (election_id, _)) in self.status.iter() {
            if election_id.is_some() {
                subscribed_list.insert(*device_id, election_id.unwrap());
            }
        }
        return subscribed_list;
    }
}
