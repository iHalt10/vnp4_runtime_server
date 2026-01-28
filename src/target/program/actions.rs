use crate::target::program::Action;
use crate::target::schema::GlobalActionSchema;
use crate::target::schema::GlobalActionsSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actions(pub Vec<Action>);

impl Actions {
    pub fn as_schema(&self) -> GlobalActionsSchema {
        let mut actions: GlobalActionsSchema = GlobalActionsSchema(Vec::new());
        for action in self.0.iter() {
            if self.is_excluded(&action.name) {
                continue;
            }
            let schema = GlobalActionSchema {
                id: action.id,
                name: action.name.clone(),
                params: action.runtime_data.as_schema(),
            };
            actions.push(schema);
        }
        return actions;
    }

    fn is_excluded(&self, name: &String) -> bool {
        return name.starts_with("act");
    }
}

impl std::ops::Deref for Actions {
    type Target = Vec<Action>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Actions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
