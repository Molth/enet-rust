#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::h_protocol::ENetProtocolCommand::ENET_PROTOCOL_COMMAND_COUNT;
use crate::h_protocol::*;

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
