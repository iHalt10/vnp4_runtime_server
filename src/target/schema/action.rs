use crate::target::driver::XilVitisNetP4Action;
use crate::target::driver::XilVitisNetP4Attribute;
use crate::target::schema::AttributeSchema;
use crate::target::schema::GlobalActionSchema;
use crate::target::schema::GlobalActionsSchema;
use crate::utils::serde::cstring as serde_cstring;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::slice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSchema {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "NameString")]
    #[serde(with = "serde_cstring")]
    pub name_string: CString,
    #[serde(rename = "ParamListSize")]
    pub param_list_size: u32,
    #[serde(rename = "ParamList")]
    pub param_list: HashMap<u32, AttributeSchema>,

    #[serde(skip)]
    param_list_raw: Option<Vec<XilVitisNetP4Attribute>>,
}

impl ActionSchema {
    pub fn from_driver_config(action: XilVitisNetP4Action, global_actions: &GlobalActionsSchema) -> Self {
        let name = ActionSchema::get_name(action);
        let global_action = global_actions.get(name.to_string_lossy().to_string()).unwrap();
        Self {
            id: global_action.id,
            name_string: name,
            param_list_size: action.ParamListSize,
            param_list: ActionSchema::get_param_list(action, global_action),
            param_list_raw: None,
        }
    }

    pub fn get_name(action: XilVitisNetP4Action) -> CString {
        unsafe {
            let c_str = CStr::from_ptr(action.NameStringPtr);
            c_str.to_owned()
        }
    }

    pub fn get_param_list(action: XilVitisNetP4Action, global_action: GlobalActionSchema) -> HashMap<u32, AttributeSchema> {
        if action.ParamListSize == 0 || action.ParamListPtr.is_null() {
            return HashMap::new();
        }
        let mut param_list: HashMap<u32, AttributeSchema> = HashMap::new();
        unsafe {
            let array = slice::from_raw_parts(action.ParamListPtr, action.ParamListSize as usize);
            for &param_raw in array.iter() {
                let param = AttributeSchema::from_driver_config(param_raw, global_action.params.clone());
                param_list.insert(param.id, param);
            }
        }
        return param_list;
    }

    pub fn to_driver_config(&mut self) -> XilVitisNetP4Action {
        if self.param_list_raw.is_none() {
            self.param_list_raw = Some(self.param_list.values().map(|param| param.to_driver_config()).collect());
        }

        XilVitisNetP4Action {
            NameStringPtr: self.name_string.as_ptr() as *const c_char,
            ParamListSize: self.param_list_size,
            ParamListPtr: self.param_list_raw.as_ref().unwrap().as_ptr() as *mut XilVitisNetP4Attribute,
        }
    }
}
