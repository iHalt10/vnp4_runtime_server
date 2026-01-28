use crate::target::driver::XilVitisNetP4AddressType;
use crate::target::driver::XilVitisNetP4EnvIf;
use crate::target::driver::XilVitisNetP4ReturnType;
use crate::target::driver::XilVitisNetP4ReturnType_XIL_VITIS_NET_P4_GENERAL_ERR_INTERNAL_ASSERTION as XIL_VITIS_NET_P4_GENERAL_ERR_INTERNAL_ASSERTION;
use crate::target::driver::XilVitisNetP4ReturnType_XIL_VITIS_NET_P4_GENERAL_ERR_NULL_PARAM as XIL_VITIS_NET_P4_GENERAL_ERR_NULL_PARAM;
use crate::target::driver::XilVitisNetP4ReturnType_XIL_VITIS_NET_P4_SUCCESS as XIL_VITIS_NET_P4_SUCCESS;
use crate::utils::mmio::Mmio;
use crate::utils::mmio::MmioConfig;
use crate::utils::mmio::MmioError;
use std::ffi::CStr;
use std::os::raw::c_void;
use tracing::debug;
use tracing::error;

pub struct UserContext {
    pub mmio: Mmio,
}

impl UserContext {
    pub fn new(config: MmioConfig) -> Result<Self, MmioError> {
        Ok(Self { mmio: Mmio::open(config)? })
    }

    pub fn to_ptr(self) -> *mut c_void {
        Box::into_raw(Box::new(self)) as *mut c_void
    }

    pub fn from_ptr<'a>(ptr: *mut c_void) -> &'a mut UserContext {
        unsafe { &mut *(ptr as *mut UserContext) }
    }

    pub fn free_ptr(ptr: *mut c_void) {
        unsafe {
            if !ptr.is_null() {
                drop(Box::from_raw(ptr as *mut UserContext));
            }
        }
    }
}

pub unsafe extern "C" fn user_word_write32(interface: *mut XilVitisNetP4EnvIf, address: XilVitisNetP4AddressType, data: u32) -> XilVitisNetP4ReturnType {
    if interface.is_null() {
        return XIL_VITIS_NET_P4_GENERAL_ERR_NULL_PARAM;
    }
    let interface: &XilVitisNetP4EnvIf = unsafe { &*interface };
    if interface.UserCtx.is_null() {
        return XIL_VITIS_NET_P4_GENERAL_ERR_INTERNAL_ASSERTION;
    }
    let user_context = UserContext::from_ptr(interface.UserCtx);
    debug!("Writing 0x{:08x} to address 0x{:08x}", data, address);

    match user_context.mmio.write32(address, data) {
        Ok(()) => XIL_VITIS_NET_P4_SUCCESS,
        Err(_) => XIL_VITIS_NET_P4_GENERAL_ERR_INTERNAL_ASSERTION,
    }
}

pub unsafe extern "C" fn user_word_read32(interface: *mut XilVitisNetP4EnvIf, address: XilVitisNetP4AddressType, data: *mut u32) -> XilVitisNetP4ReturnType {
    if interface.is_null() || data.is_null() {
        return XIL_VITIS_NET_P4_GENERAL_ERR_NULL_PARAM;
    }
    let interface: &XilVitisNetP4EnvIf = unsafe { &*interface };
    if interface.UserCtx.is_null() {
        return XIL_VITIS_NET_P4_GENERAL_ERR_INTERNAL_ASSERTION;
    }

    let user_context = UserContext::from_ptr(interface.UserCtx);
    match user_context.mmio.read32(address) {
        Ok(value) => {
            unsafe {
                *data = value;
            }
            debug!("Read 0x{:08x} from address 0x{:08x}", value, address);
            XIL_VITIS_NET_P4_SUCCESS
        }
        Err(e) => {
            error!("Read failed from address 0x{:08x}: {}", address, e);
            XIL_VITIS_NET_P4_GENERAL_ERR_INTERNAL_ASSERTION
        }
    }
}

pub unsafe extern "C" fn user_log(interface: *mut XilVitisNetP4EnvIf, message: *const std::os::raw::c_char) -> XilVitisNetP4ReturnType {
    if interface.is_null() {
        return XIL_VITIS_NET_P4_GENERAL_ERR_NULL_PARAM;
    }

    if message.is_null() {
        return XIL_VITIS_NET_P4_GENERAL_ERR_NULL_PARAM;
    }

    let msg = unsafe { CStr::from_ptr(message) };

    match msg.to_str() {
        Ok(s) => {
            debug!("VitisNetP4 Log: {}", s);
            XIL_VITIS_NET_P4_SUCCESS
        }
        Err(e) => {
            error!("Failed to convert C string to Rust string: {}", e);
            XIL_VITIS_NET_P4_GENERAL_ERR_INTERNAL_ASSERTION
        }
    }
}
