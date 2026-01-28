use crate::target::driver::XilVitisNetP4ReturnType;
use crate::target::driver::XilVitisNetP4ReturnTypeToString;
use std::ffi::CStr;

pub fn code_to_name(code: XilVitisNetP4ReturnType) -> String {
    let c_str_ptr = unsafe { XilVitisNetP4ReturnTypeToString(code) };

    if c_str_ptr.is_null() {
        return "Unknown Code".to_string();
    }

    unsafe { CStr::from_ptr(c_str_ptr).to_str().map(|s| s.to_string()).unwrap_or_else(|_| "Unknown Code".to_string()) }
}
