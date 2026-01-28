use crate::target::driver::XilVitisNetP4TargetConfig;
use crate::target::driver::XilVitisNetP4TargetCounterConfig;
use crate::target::driver::XilVitisNetP4TargetRegisterConfig;
use crate::target::driver::XilVitisNetP4TargetTableConfig;
use crate::target::program::Program;
use crate::target::schema::GlobalActionsSchema;
use crate::target::schema::TargetBuildInfoConfigSchema;
use crate::target::schema::TargetCounterConfigSchema;
use crate::target::schema::TargetCtrlConfigSchema;
use crate::target::schema::TargetInterruptConfigSchema;
use crate::target::schema::TargetRegisterConfigSchema;
use crate::target::schema::TargetTableConfigSchema;
use crate::target::schema::TargetTableConfigsSchema;
use crate::utils::serde::JsonError;
use p4runtime::p4::config::v1::P4Info;
use p4runtime::p4::config::v1::PkgInfo;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::ptr::null_mut;
use std::slice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfigSchema {
    #[serde(rename = "Endian")]
    pub endian: u32,
    #[serde(rename = "TableListSize")]
    pub table_list_size: u32,
    #[serde(rename = "TableList")]
    pub table_list: TargetTableConfigsSchema,
    #[serde(rename = "CounterListSize")]
    pub counter_list_size: u32,
    #[serde(rename = "CounterList")]
    pub counter_list: Vec<TargetCounterConfigSchema>,
    #[serde(rename = "RegisterListSize")]
    pub register_list_size: u32,
    #[serde(rename = "RegisterList")]
    pub register_list: Vec<TargetRegisterConfigSchema>,
    #[serde(rename = "BuildInfo")]
    pub build_info: Option<TargetBuildInfoConfigSchema>,
    #[serde(rename = "Interrupt")]
    pub interrupt: Option<TargetInterruptConfigSchema>,
    #[serde(rename = "CtrlConfig")]
    pub ctrl_config: Option<TargetCtrlConfigSchema>,
    #[serde(rename = "GlobalActions")]
    pub actions: GlobalActionsSchema,

    #[serde(skip)]
    table_list_raw: Option<Vec<XilVitisNetP4TargetTableConfig>>,
    #[serde(skip)]
    table_ptr_list: Option<Vec<*mut XilVitisNetP4TargetTableConfig>>,
    #[serde(skip)]
    counter_list_raw: Option<Vec<XilVitisNetP4TargetCounterConfig>>,
    #[serde(skip)]
    counter_ptr_list: Option<Vec<*mut XilVitisNetP4TargetCounterConfig>>,
    #[serde(skip)]
    register_list_raw: Option<Vec<XilVitisNetP4TargetRegisterConfig>>,
    #[serde(skip)]
    register_ptr_list: Option<Vec<*mut XilVitisNetP4TargetRegisterConfig>>,
}

impl TargetConfigSchema {
    pub fn from_driver_config(config: XilVitisNetP4TargetConfig, program: Program) -> Self {
        Self {
            endian: config.Endian,
            table_list_size: config.TableListSize,
            table_list: TargetConfigSchema::get_table_list(config, &program),
            counter_list_size: config.CounterListSize,
            counter_list: TargetConfigSchema::get_counter_list(config),
            register_list_size: config.RegisterListSize,
            register_list: TargetConfigSchema::get_register_list(config),
            build_info: TargetConfigSchema::get_build_info(config),
            interrupt: TargetConfigSchema::get_interrupt(config),
            ctrl_config: TargetConfigSchema::get_ctrl_config(config),
            actions: program.actions.as_schema(),
            table_list_raw: None,
            table_ptr_list: None,
            counter_list_raw: None,
            counter_ptr_list: None,
            register_list_raw: None,
            register_ptr_list: None,
        }
    }

    pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Self, JsonError> {
        let path = path.as_ref();
        let file = File::open(&path).map_err(|e| JsonError::FileRead { path: path.to_path_buf(), source: e })?;
        let reader = BufReader::new(file);
        let config: Self = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn save_json<P: AsRef<Path>>(&self, path: P) -> Result<(), JsonError> {
        let path = path.as_ref();
        let file = File::create(&path).map_err(|e| JsonError::FileWrite { path: path.to_path_buf(), source: e })?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    pub fn get_table_list(config: XilVitisNetP4TargetConfig, program: &Program) -> TargetTableConfigsSchema {
        if config.TableListSize == 0 || config.TableListPtr.is_null() {
            return TargetTableConfigsSchema(HashMap::new());
        }
        let mut table_list = TargetTableConfigsSchema(HashMap::new());
        unsafe {
            let array = slice::from_raw_parts(config.TableListPtr, config.TableListSize as usize);
            for &table_raw in array.iter() {
                let table = TargetTableConfigSchema::from_driver_config(*table_raw, program);
                table_list.insert(table.id, table);
            }
        }
        return table_list;
    }

    pub fn get_counter_list(config: XilVitisNetP4TargetConfig) -> Vec<TargetCounterConfigSchema> {
        if config.CounterListSize == 0 || config.CounterListPtr.is_null() {
            return Vec::new();
        }
        let mut counter_list = Vec::with_capacity(config.CounterListSize as usize);
        unsafe {
            let array = slice::from_raw_parts(config.CounterListPtr, config.CounterListSize as usize);
            for &counter_raw in array.iter() {
                counter_list.push(TargetCounterConfigSchema::from_driver_config(*counter_raw));
            }
        }
        return counter_list;
    }

    pub fn get_register_list(config: XilVitisNetP4TargetConfig) -> Vec<TargetRegisterConfigSchema> {
        if config.RegisterListSize == 0 || config.RegisterListPtr.is_null() {
            return Vec::new();
        }
        let mut register_list = Vec::with_capacity(config.RegisterListSize as usize);
        unsafe {
            let array = slice::from_raw_parts(config.RegisterListPtr, config.RegisterListSize as usize);
            for &register_raw in array.iter() {
                register_list.push(TargetRegisterConfigSchema::from_driver_config(*register_raw));
            }
        }
        return register_list;
    }

    pub fn get_build_info(config: XilVitisNetP4TargetConfig) -> Option<TargetBuildInfoConfigSchema> {
        unsafe {
            if config.BuildInfoPtr.is_null() {
                return None;
            }
            return Some(TargetBuildInfoConfigSchema::from_driver_config(*config.BuildInfoPtr));
        }
    }

    pub fn get_interrupt(_: XilVitisNetP4TargetConfig) -> Option<TargetInterruptConfigSchema> {
        return None;
    }

    pub fn get_ctrl_config(config: XilVitisNetP4TargetConfig) -> Option<TargetCtrlConfigSchema> {
        unsafe {
            if config.CtrlConfigPtr.is_null() {
                return None;
            }
            return Some(TargetCtrlConfigSchema::from_driver_config(*config.CtrlConfigPtr));
        }
    }

    pub fn to_driver_config(&mut self) -> XilVitisNetP4TargetConfig {
        if self.table_list_raw.is_none() {
            self.table_list_raw = Some(self.table_list.values_mut().map(|table| table.to_driver_config()).collect());
        }

        if self.table_ptr_list.is_none() {
            self.table_ptr_list = Some(self.table_list_raw.as_mut().unwrap().iter_mut().map(|table| table as *mut XilVitisNetP4TargetTableConfig).collect());
        }

        if self.counter_list_raw.is_none() {
            self.counter_list_raw = Some(self.counter_list.iter_mut().map(|counter| counter.to_driver_config()).collect());
        }

        if self.counter_ptr_list.is_none() {
            self.counter_ptr_list = Some(
                self.counter_list_raw
                    .as_mut()
                    .unwrap()
                    .iter_mut()
                    .map(|counter| counter as *mut XilVitisNetP4TargetCounterConfig)
                    .collect(),
            );
        }

        if self.register_list_raw.is_none() {
            self.register_list_raw = Some(self.register_list.iter_mut().map(|register| register.to_driver_config()).collect());
        }

        if self.register_ptr_list.is_none() {
            self.register_ptr_list = Some(
                self.register_list_raw
                    .as_mut()
                    .unwrap()
                    .iter_mut()
                    .map(|table| table as *mut XilVitisNetP4TargetRegisterConfig)
                    .collect(),
            );
        }

        XilVitisNetP4TargetConfig {
            Endian: self.endian,
            TableListSize: self.table_list_size,
            TableListPtr: self.table_ptr_list.as_ref().unwrap().as_ptr() as *mut *mut XilVitisNetP4TargetTableConfig,
            CounterListSize: self.counter_list_size,
            CounterListPtr: self.counter_ptr_list.as_ref().unwrap().as_ptr() as *mut *mut XilVitisNetP4TargetCounterConfig,
            RegisterListSize: self.register_list_size,
            RegisterListPtr: self.register_ptr_list.as_ref().unwrap().as_ptr() as *mut *mut XilVitisNetP4TargetRegisterConfig,
            BuildInfoPtr: self.build_info.as_ref().map(|v| v.to_driver_config()).map(|c| Box::into_raw(Box::new(c))).unwrap_or(null_mut()),
            InterruptPtr: null_mut(),
            CtrlConfigPtr: self.ctrl_config.as_ref().map(|v| v.to_driver_config()).map(|c| Box::into_raw(Box::new(c))).unwrap_or(null_mut()),
        }
    }

    pub fn as_p4info(&self) -> P4Info {
        P4Info {
            pkg_info: Some(PkgInfo {
                name: "".to_string(),
                version: "".to_string(),
                doc: None,
                annotations: Vec::new(),
                annotation_locations: Vec::new(),
                arch: "xsa".to_string(),
                organization: "".to_string(),
                contact: "".to_string(),
                url: "".to_string(),
                structured_annotations: Vec::new(),
                platform_properties: None,
            }),
            tables: self.table_list.as_p4info(),
            actions: self.actions.as_p4info(),
            action_profiles: Vec::new(),
            counters: Vec::new(),
            direct_counters: Vec::new(),
            meters: Vec::new(),
            direct_meters: Vec::new(),
            controller_packet_metadata: Vec::new(),
            value_sets: Vec::new(),
            registers: Vec::new(),
            digests: Vec::new(),
            externs: Vec::new(),
            type_info: None,
        }
    }
}
