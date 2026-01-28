use p4runtime::p4::v1::Uint128;

pub mod get_forwarding_pipeline_config_request {
    use p4runtime::p4::v1::get_forwarding_pipeline_config_request::ResponseType;
    pub fn as_response_type(value: i32) -> Option<ResponseType> {
        match value {
            0 => Some(ResponseType::All),
            1 => Some(ResponseType::CookieOnly),
            2 => Some(ResponseType::P4infoAndCookie),
            3 => Some(ResponseType::DeviceConfigAndCookie),
            _ => None,
        }
    }
}

pub fn as_uint128_from(value: u128) -> Uint128 {
    Uint128 {
        high: (value >> 64) as u64,
        low: value as u64,
    }
}

pub fn as_u128_from(value: &Uint128) -> u128 {
    ((value.high as u128) << 64) | (value.low as u128)
}
