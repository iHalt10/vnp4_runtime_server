use crate::target::driver::XilVitisNetP4CamConfig;
use crate::utils::serde::cstring as serde_cstring;
use serde::Deserialize;
use serde::Serialize;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamConfigSchema {
    #[serde(rename = "BaseAddr")]
    pub base_addr: usize,
    #[serde(rename = "FormatString")]
    #[serde(with = "serde_cstring")]
    pub format_string: CString,
    #[serde(rename = "NumEntries")]
    pub num_entries: u32,
    #[serde(rename = "RamFrequencyHz")]
    pub ram_frequency_hz: u32,
    #[serde(rename = "LookupFrequencyHz")]
    pub lookup_frequency_hz: u32,
    #[serde(rename = "LookupsPerSec")]
    pub lookups_per_sec: u32,
    #[serde(rename = "ResponseSizeBits")]
    pub response_size_bits: u16,
    #[serde(rename = "PrioritySizeBits")]
    pub priority_size_bits: u8,
    #[serde(rename = "NumMasks")]
    pub num_masks: u8,
    #[serde(rename = "Endian")]
    pub endian: u32,
    #[serde(rename = "MemType")]
    pub mem_type: u32,
    #[serde(rename = "RamSizeKbytes")]
    pub ram_size_kbytes: u32,
    #[serde(rename = "OptimizationType")]
    pub optimization_type: u32,
    #[serde(rename = "RamChannelWidth")]
    pub ram_channel_width: u32,
    #[serde(rename = "RamNumBanks")]
    pub ram_num_banks: u32,
    #[serde(rename = "CamHWUpdateEnable")]
    pub cam_hw_update_enable: u8,
    #[serde(rename = "CamVariableRate")]
    pub cam_variable_rate: u8,
    #[serde(rename = "CamTplLookup")]
    pub cam_tpl_lookup: u8,
}

impl CamConfigSchema {
    pub fn from_driver_config(config: XilVitisNetP4CamConfig) -> Self {
        Self {
            base_addr: config.BaseAddr,
            format_string: CamConfigSchema::get_format_string(config),
            num_entries: config.NumEntries,
            ram_frequency_hz: config.RamFrequencyHz,
            lookup_frequency_hz: config.LookupFrequencyHz,
            lookups_per_sec: config.LookupsPerSec,
            response_size_bits: config.ResponseSizeBits,
            priority_size_bits: config.PrioritySizeBits,
            num_masks: config.NumMasks,
            endian: config.Endian,
            mem_type: config.MemType,
            ram_size_kbytes: config.RamSizeKbytes,
            optimization_type: config.OptimizationType,
            ram_channel_width: config.RamChannelWidth,
            ram_num_banks: config.RamNumBanks,
            cam_hw_update_enable: config.CamHWUpdateEnable,
            cam_variable_rate: config.CamVariableRate,
            cam_tpl_lookup: config.CamTplLookup,
        }
    }

    pub fn get_format_string(config: XilVitisNetP4CamConfig) -> CString {
        unsafe {
            let c_str = CStr::from_ptr(config.FormatStringPtr);
            c_str.to_owned()
        }
    }

    pub fn to_driver_config(&self) -> XilVitisNetP4CamConfig {
        XilVitisNetP4CamConfig {
            BaseAddr: self.base_addr,
            FormatStringPtr: self.format_string.as_ptr() as *mut c_char,
            NumEntries: self.num_entries,
            RamFrequencyHz: self.ram_frequency_hz,
            LookupFrequencyHz: self.lookup_frequency_hz,
            LookupsPerSec: self.lookups_per_sec,
            ResponseSizeBits: self.response_size_bits,
            PrioritySizeBits: self.priority_size_bits,
            NumMasks: self.num_masks,
            Endian: self.endian,
            MemType: self.mem_type,
            RamSizeKbytes: self.ram_size_kbytes,
            OptimizationType: self.optimization_type,
            RamChannelWidth: self.ram_channel_width,
            RamNumBanks: self.ram_num_banks,
            CamHWUpdateEnable: self.cam_hw_update_enable,
            CamVariableRate: self.cam_variable_rate,
            CamTplLookup: self.cam_tpl_lookup,
        }
    }
}
