#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub const INVALID_SOCKET: isize = !0;

pub const ENET_SOCKET_NULL: isize = INVALID_SOCKET;

pub fn ENET_HOST_TO_NET_16(host: u16) -> u16 {
    host.to_be()
}

pub fn ENET_HOST_TO_NET_32(host: u32) -> u32 {
    host.to_be()
}

pub fn ENET_NET_TO_HOST_16(network: u16) -> u16 {
    u16::from_be(network)
}

pub fn ENET_NET_TO_HOST_32(network: u32) -> u32 {
    u32::from_be(network)
}

#[derive(Copy, Clone, Default)]
pub struct ENetBuffer {
    pub dataID: usize,
    pub dataLength: usize,
}

#[macro_export]
macro_rules! enet_buffer_as_slice {
    ($buffer:expr, $data:expr) => {
        &$data[$buffer.dataID][..$buffer.dataLength]
    };
}

#[macro_export]
macro_rules! enet_buffer_as_mut_slice {
    ($buffer:expr, $data:expr) => {
        &mut $data[$buffer.dataID][..$buffer.dataLength]
    };
}
