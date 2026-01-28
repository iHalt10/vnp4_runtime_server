use crate::target::driver::XilVitisNetP4TargetCounterConfig;
use crate::target::schema::CounterConfigSchema;
use crate::utils::serde::cstring as serde_cstring;
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetCounterConfigSchema {
    #[serde(rename = "NameString")]
    #[serde(with = "serde_cstring")]
    pub name_string: CString,
    #[serde(rename = "Config")]
    pub config: CounterConfigSchema,
}

impl TargetCounterConfigSchema {
    pub fn from_driver_config(counter: XilVitisNetP4TargetCounterConfig) -> Self {
        Self {
            name_string: TargetCounterConfigSchema::get_name(counter),
            config: CounterConfigSchema::from_driver_config(counter.Config),
        }
    }

    pub fn get_name(counter: XilVitisNetP4TargetCounterConfig) -> CString {
        unsafe {
            let c_str = CStr::from_ptr(counter.NameStringPtr);
            c_str.to_owned()
        }
    }

    pub fn to_driver_config(&mut self) -> XilVitisNetP4TargetCounterConfig {
        XilVitisNetP4TargetCounterConfig {
            NameStringPtr: self.name_string.as_ptr() as *const c_char,
            Config: self.config.to_driver_config(),
        }
    }
}
