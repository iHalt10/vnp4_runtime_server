use crate::target::driver::XilVitisNetP4Action;
use crate::target::driver::XilVitisNetP4TableConfig;
use crate::target::schema::ActionSchema;
use crate::target::schema::CamConfigSchema;
use crate::target::schema::GlobalActionsSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::slice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConfigSchema {
    #[serde(rename = "Endian")]
    pub endian: u32,
    #[serde(rename = "Mode")]
    pub mode: u32,
    #[serde(rename = "KeySizeBits")]
    pub key_size_bits: u32,
    #[serde(rename = "CamConfig")]
    pub cam_config: CamConfigSchema,
    #[serde(rename = "ActionIdWidthBits")]
    pub action_id_width_bits: u32,
    #[serde(rename = "ActionListSize")]
    pub action_list_size: u32,
    #[serde(rename = "ActionList")]
    pub action_list: HashMap<u32, ActionSchema>,

    #[serde(skip)]
    action_list_raw: Option<Vec<XilVitisNetP4Action>>,
    #[serde(skip)]
    action_ptr_list: Option<Vec<*mut XilVitisNetP4Action>>,
}

impl TableConfigSchema {
    pub fn from_driver_config(config: XilVitisNetP4TableConfig, global_actions: GlobalActionsSchema) -> Self {
        Self {
            endian: config.Endian,
            mode: config.Mode,
            key_size_bits: config.KeySizeBits,
            cam_config: CamConfigSchema::from_driver_config(config.CamConfig),
            action_id_width_bits: config.ActionIdWidthBits,
            action_list_size: config.ActionListSize,
            action_list: TableConfigSchema::get_action_list(config, global_actions),
            action_list_raw: None,
            action_ptr_list: None,
        }
    }

    pub fn get_action_list(config: XilVitisNetP4TableConfig, global_actions: GlobalActionsSchema) -> HashMap<u32, ActionSchema> {
        if config.ActionListSize == 0 || config.ActionListPtr.is_null() {
            return HashMap::new();
        }
        let mut action_list: HashMap<u32, ActionSchema> = HashMap::new();
        unsafe {
            let array = slice::from_raw_parts(config.ActionListPtr, config.ActionListSize as usize);
            for &action_raw in array.iter() {
                let action = ActionSchema::from_driver_config(*action_raw, &global_actions);
                action_list.insert(action.id, action);
            }
        }
        return action_list;
    }

    pub fn to_driver_config(&mut self) -> XilVitisNetP4TableConfig {
        if self.action_list_raw.is_none() {
            self.action_list_raw = Some(self.action_list.values_mut().map(|action| action.to_driver_config()).collect());
        }

        if self.action_ptr_list.is_none() {
            self.action_ptr_list = Some(self.action_list_raw.as_mut().unwrap().iter_mut().map(|action| action as *mut XilVitisNetP4Action).collect());
        }

        XilVitisNetP4TableConfig {
            Endian: self.endian,
            Mode: self.mode,
            KeySizeBits: self.key_size_bits,
            CamConfig: self.cam_config.to_driver_config(),
            ActionIdWidthBits: self.action_id_width_bits,
            ActionListSize: self.action_list_size,
            ActionListPtr: self.action_ptr_list.as_ref().unwrap().as_ptr() as *mut *mut XilVitisNetP4Action,
        }
    }
}
