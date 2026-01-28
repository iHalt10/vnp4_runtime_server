use super::error::DeviceError;
use crate::server::config::DeviceConfig;
use crate::server::subscribers::Subscribers;
use crate::target::driver::XilVitisNetP4EnvIf;
use crate::target::driver::XilVitisNetP4ReturnType_XIL_VITIS_NET_P4_SUCCESS as XIL_VITIS_NET_P4_SUCCESS;
use crate::target::driver::XilVitisNetP4TargetConfig;
use crate::target::driver::XilVitisNetP4TargetCtx;
use crate::target::driver::XilVitisNetP4TargetExit;
use crate::target::driver::XilVitisNetP4TargetInit;
use crate::target::models::Table;
use crate::target::schema::TargetConfigSchema;
use crate::target::user_context::UserContext;
use crate::target::user_context::user_log;
use crate::target::user_context::user_word_read32;
use crate::target::user_context::user_word_write32;
use crate::utils::driver::code_to_name;
use p4runtime::p4::config::v1::P4Info;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;

#[derive(Debug)]
pub struct Device {
    pub id: u64,
    pub tables: Arc<HashMap<u32, RwLock<Table>>>,
    pub interface: XilVitisNetP4EnvIf,
    pub target_schema: TargetConfigSchema,
    pub target_config: XilVitisNetP4TargetConfig,
    pub target_context: XilVitisNetP4TargetCtx,
    pub subscribers: RwLock<Subscribers>,
}

impl Device {
    pub fn open(config: DeviceConfig) -> Result<Self, DeviceError> {
        let mut interface = XilVitisNetP4EnvIf {
            UserCtx: UserContext::new(config.mmio)?.to_ptr(),
            WordWrite32: Some(user_word_write32),
            WordRead32: Some(user_word_read32),
            LogError: Some(user_log),
            LogInfo: Some(user_log),
            DebugFlags: 0,
        };
        let mut target_schema = TargetConfigSchema::load_json(config.target_config)?;
        let mut target_config = target_schema.to_driver_config();
        let mut target_context = XilVitisNetP4TargetCtx::default();
        let code = unsafe {
            XilVitisNetP4TargetInit(
                &mut target_context as *mut XilVitisNetP4TargetCtx,
                &mut interface as *mut XilVitisNetP4EnvIf,
                &mut target_config as *mut XilVitisNetP4TargetConfig,
            )
        };
        if code != XIL_VITIS_NET_P4_SUCCESS {
            let user_context = UserContext::from_ptr(interface.UserCtx);
            user_context.mmio.close()?;
            UserContext::free_ptr(interface.UserCtx);
            interface.UserCtx = std::ptr::null_mut();
            return Err(DeviceError::Driver { name: code_to_name(code), code: code });
        }
        let mut tables: HashMap<u32, RwLock<Table>> = HashMap::new();
        for table_schema in target_schema.table_list.values() {
            let table = Table::new(table_schema.clone(), &mut target_context)?;
            tables.insert(table_schema.id, RwLock::new(table));
        }
        Ok(Self {
            id: config.id,
            interface: interface,
            target_schema: target_schema,
            target_config: target_config,
            target_context: target_context,
            tables: Arc::new(tables),
            subscribers: RwLock::new(Subscribers::new(config.id)),
        })
    }

    pub fn as_p4info(&self) -> P4Info {
        self.target_schema.as_p4info()
    }

    pub fn close(&mut self) -> Result<(), DeviceError> {
        unsafe { XilVitisNetP4TargetExit(&mut self.target_context as *mut XilVitisNetP4TargetCtx) };
        let user_context = UserContext::from_ptr(self.interface.UserCtx);
        user_context.mmio.close()?;
        UserContext::free_ptr(self.interface.UserCtx);
        self.interface.UserCtx = std::ptr::null_mut();
        Ok(())
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            warn!("Error during device {} cleanup: {}", self.id, e);
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}
