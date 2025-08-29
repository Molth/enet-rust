#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::c_win32::*;
use crate::h_enet::*;
use crate::h_protocol::*;
use crate::h_win32::*;

pub fn enet_host_ping(host: &ENetHost, address: &ENetAddress) -> bool {
    let mut data = [0u8; 1];
    let buffer = ENetBuffer { data: &mut data };
    if enet_socket_send(&host.socket, address, &[buffer], 1) > 0 {
        true
    } else {
        false
    }
}

pub fn enet_host_create(
    address: &ENetAddress,
    peerCount: usize,
    mut channelLimit: usize,
    incomingBandwidth: u32,
    outgoingBandwidth: u32,
) -> Option<Box<ENetHost>> {
    if peerCount > ENET_PROTOCOL_MAXIMUM_PEER_ID as usize {
        return None;
    }

    let socket = match enet_socket_create(address) {
        Ok(x) => x,
        Err(_) => return None,
    };

    let _ = enet_socket_set_option(&socket, ENetSocketOption::ENET_SOCKOPT_NONBLOCK, 1);
    let _ = enet_socket_set_option(&socket, ENetSocketOption::ENET_SOCKOPT_BROADCAST, 1);

    let _ = enet_socket_set_option(
        &socket,
        ENetSocketOption::ENET_SOCKOPT_RCVBUF,
        ENET_HOST_RECEIVE_BUFFER_SIZE as i32,
    );

    let _ = enet_socket_set_option(
        &socket,
        ENetSocketOption::ENET_SOCKOPT_SNDBUF,
        ENET_HOST_SEND_BUFFER_SIZE as i32,
    );

    if !(channelLimit != 0) || channelLimit > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as usize {
        channelLimit = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as usize;
    } else if channelLimit < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as usize {
        channelLimit = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as usize;
    }

    None
}
