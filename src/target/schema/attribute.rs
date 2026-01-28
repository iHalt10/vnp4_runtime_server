use crate::target::driver::XilVitisNetP4Attribute;
use crate::target::schema::GlobalParamsSchema;
use crate::utils::serde::cstring as serde_cstring;
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeSchema {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "NameString")]
    #[serde(with = "serde_cstring")]
    pub name_string: CString,
    #[serde(rename = "Value")]
    pub value: u32,
}

impl AttributeSchema {
    pub fn from_driver_config(attribute: XilVitisNetP4Attribute, global_params: GlobalParamsSchema) -> Self {
        let name = AttributeSchema::get_name(attribute);
        let global_param = global_params.get(name.to_string_lossy().to_string()).unwrap();
        Self {
            id: global_param.id,
            name_string: name,
            value: attribute.Value,
        }
    }

    pub fn get_name(attribute: XilVitisNetP4Attribute) -> CString {
        unsafe {
            let c_str = CStr::from_ptr(attribute.NameStringPtr);
            c_str.to_owned()
        }
    }

    pub fn to_driver_config(&self) -> XilVitisNetP4Attribute {
        XilVitisNetP4Attribute {
            NameStringPtr: self.name_string.as_ptr() as *const c_char,
            Value: self.value,
        }
    }
}
