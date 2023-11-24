use deku::prelude::*;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Request {
    #[deku(bits = 64, endian = "little")]
    pub payload_size: usize,
    #[deku(endian = "little")]
    pub period_millis: u64,
    #[deku(bits = 64, endian = "little")]
    pub count: usize,
    #[deku(bits = 4, endian = "little")]
    pub num_priorities: usize,
    #[deku(count = "num_priorities")]
    pub priorities: Vec<i32>,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum Response {
    #[deku(id = "0")]
    Ready,
    #[deku(id = "1")]
    Abort,
}
