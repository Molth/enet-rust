#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::enet_host_get_mut_peer;
use crate::h_enet::ENetPeerFlag::*;
use crate::h_enet::*;
use crate::h_protocol::ENetProtocolCommand::*;
use crate::h_protocol::*;
use std::collections::VecDeque;

pub const commandSizes: [usize; ENET_PROTOCOL_COMMAND_COUNT as usize] = [
    0,
    size_of::<ENetProtocolAcknowledge>(),
    size_of::<ENetProtocolConnect>(),
    size_of::<ENetProtocolVerifyConnect>(),
    size_of::<ENetProtocolDisconnect>(),
    size_of::<ENetProtocolPing>(),
    size_of::<ENetProtocolSendReliable>(),
    size_of::<ENetProtocolSendUnreliable>(),
    size_of::<ENetProtocolSendFragment>(),
    size_of::<ENetProtocolSendUnsequenced>(),
    size_of::<ENetProtocolBandwidthLimit>(),
    size_of::<ENetProtocolThrottleConfigure>(),
    size_of::<ENetProtocolSendFragment>(),
];

pub fn enet_protocol_command_size(commandNumber: u8) -> usize {
    commandSizes[((commandNumber as i32) & (ENET_PROTOCOL_COMMAND_MASK as i32)) as usize]
}

pub fn enet_protocol_change_state(host: &mut ENetHost, incomingPeerID: u16, state: ENetPeerState) {
    todo!()
}

pub fn enet_protocol_dispatch_state(
    host: &mut ENetHost,
    incomingPeerID: u16,
    state: ENetPeerState,
) {
    enet_protocol_change_state(host, incomingPeerID, state);

    let peer = enet_host_get_mut_peer!(host, incomingPeerID);
    if !(((peer.flags as u32) & (ENET_PEER_FLAG_NEEDS_DISPATCH as u32)) != 0) {
        host.dispatchQueue.push_back(peer.incomingPeerID);
        let mut flags = peer.flags as u32;
        flags |= ENET_PEER_FLAG_NEEDS_DISPATCH as u32;
        peer.flags = flags as u16;
    }
}

pub fn enet_protocol_dispatch_incoming_commands(host: &ENetHost, event: &mut ENetEvent) {
    todo!()
}

pub fn enet_protocol_notify_connect(
    host: &mut ENetHost,
    incomingPeerID: u16,
    event: &mut ENetEvent,
) {
    todo!()
}

pub fn enet_protocol_notify_disconnect(
    host: &mut ENetHost,
    incomingPeerID: u16,
    event: &mut ENetEvent,
) {
    todo!()
}

pub fn enet_protocol_remove_sent_unreliable_commands(
    host: &mut ENetHost,
    incomingPeerID: u16,
    sentUnreliableCommands: &mut VecDeque<ENetOutgoingCommand>,
) {
    todo!()
}

pub fn enet_protocol_find_sent_reliable_command(
    list: &VecDeque<ENetOutgoingCommand>,
    reliableSequenceNumber: u16,
    channelID: u8,
) -> Option<usize> {
    todo!()
}

pub fn enet_protocol_remove_sent_reliable_command(
    host: &mut ENetHost,
    incomingPeerID: u16,
    reliableSequenceNumber: u16,
    channelID: u8,
) -> ENetProtocolCommand {
    todo!()
}
