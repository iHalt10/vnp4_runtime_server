use crate::target::schema::GlobalActionSchema;
use p4runtime::p4::config::v1::Action as P4RuntimeAction;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalActionsSchema(pub Vec<GlobalActionSchema>);

impl GlobalActionsSchema {
    pub fn get(&self, name: String) -> Option<GlobalActionSchema> {
        for action in self.0.iter() {
            if action.name.ends_with(&name) {
                return Some(action.clone());
            }
        }
        return None;
    }

    pub fn as_p4info(&self) -> Vec<P4RuntimeAction> {
        return self.0.iter().map(|schema| schema.as_p4info()).collect();
    }
}

impl std::ops::Deref for GlobalActionsSchema {
    type Target = Vec<GlobalActionSchema>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GlobalActionsSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
