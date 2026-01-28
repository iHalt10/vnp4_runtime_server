use crate::target::driver::XilVitisNetP4TargetRegisterConfig;
use crate::target::schema::RegisterConfigSchema;
use crate::utils::serde::cstring as serde_cstring;
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetRegisterConfigSchema {
    #[serde(rename = "NameString")]
    #[serde(with = "serde_cstring")]
    pub name_string: CString,
    #[serde(rename = "Config")]
    pub config: RegisterConfigSchema,
}

impl TargetRegisterConfigSchema {
    pub fn from_driver_config(register: XilVitisNetP4TargetRegisterConfig) -> Self {
        Self {
            name_string: TargetRegisterConfigSchema::get_name(register),
            config: RegisterConfigSchema::from_driver_config(register.Config),
        }
    }

    pub fn get_name(register: XilVitisNetP4TargetRegisterConfig) -> CString {
        unsafe {
            let c_str = CStr::from_ptr(register.NameStringPtr);
            c_str.to_owned()
        }
    }

    pub fn to_driver_config(&mut self) -> XilVitisNetP4TargetRegisterConfig {
        XilVitisNetP4TargetRegisterConfig {
            NameStringPtr: self.name_string.as_ptr() as *const c_char,
            Config: self.config.to_driver_config(),
        }
    }
}
