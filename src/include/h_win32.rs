use std::mem::zeroed;

pub const INVALID_SOCKET: isize = !0;

pub const ENET_SOCKET_NULL: isize = INVALID_SOCKET;

pub fn enet_host_to_net_16(host: u16) -> u16 {
    host.to_be()
}

pub fn enet_host_to_net_32(host: u32) -> u32 {
    host.to_be()
}

pub fn enet_net_to_host_16(network: u16) -> u16 {
    u16::from_be(network)
}

pub fn enet_net_to_host_32(network: u32) -> u32 {
    u32::from_be(network)
}

pub struct ENetBuffer<'a> {
    pub data: &'a mut [u8],
}

pub struct ENetPacket {
    pub data: Vec<u8>,
}
