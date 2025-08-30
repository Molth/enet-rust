use crate::h_enet::ENetPacket;
use crate::h_enet::ENetPacketFlag::ENET_PACKET_FLAG_NO_ALLOCATE;
use std::cell::RefCell;
use std::rc::Rc;

pub fn enet_packet_create(data: Rc<RefCell<Box<[u8]>>>, flags: u32) -> ENetPacket {
    let mut packet = ENetPacket::default();
    if (flags & ENET_PACKET_FLAG_NO_ALLOCATE as u32) != 0 {}

    packet
}
