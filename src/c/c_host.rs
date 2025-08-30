#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::c_win32::*;
use crate::h_enet::*;
use crate::h_protocol::*;
use crate::h_win32::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn enet_host_ping(host: &ENetHost, address: &ENetAddress) -> bool {
    let data: [u8; 1] = [0u8; 1];
    let data_slices: [&[u8]; 1] = [&data[..]];

    let buffer = ENetBuffer {
        dataID: 0,
        dataLength: 1,
    };

    if enet_socket_send(&host.socket, address, &data_slices, &[buffer], 1) > 0 {
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
) -> Option<Rc<RefCell<ENetHost>>> {
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

    if !(channelLimit != 0) || channelLimit > (ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as usize) {
        channelLimit = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as usize;
    } else if channelLimit < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as usize {
        channelLimit = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as usize;
    }

    let mut host = ENetHost {
        socket,
        address: *address,
        incomingBandwidth,
        outgoingBandwidth,
        bandwidthThrottleEpoch: 0,
        mtu: ENET_HOST_DEFAULT_MTU,
        randomSeed: 0,
        recalculateBandwidthLimits: 0,
        peers: Box::default(),
        channelLimit,
        serviceTime: 0,
        dispatchQueue: VecDeque::new(),
        totalQueued: 0,
        packetSize: 0,
        headerFlags: 0,
        commands: [ENetProtocol::default(); ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS as usize],
        commandCount: 0,
        buffers: [ENetBuffer::default(); ENET_BUFFER_MAXIMUM as usize],
        bufferCount: 0,
        checksum: None,
        compressor: ENetCompressor::new(),
        packetData: [[0u8; ENET_PROTOCOL_MAXIMUM_MTU as usize]; 2],
        receivedAddress: ENetAddress::new(),
        receivedData: 0,
        receivedDataLength: 0,
        totalSentData: 0,
        totalSentPackets: 0,
        totalReceivedData: 0,
        totalReceivedPackets: 0,
        intercept: None,
        connectedPeers: 0,
        bandwidthLimitedPeers: 0,
        duplicatePeers: ENET_PROTOCOL_MAXIMUM_PEER_ID as usize,
        maximumPacketSize: ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE as usize,
        maximumWaitingData: ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA as usize,
    };

    host.randomSeed = ((&host as *const _) as usize) as u32;
    host.randomSeed = host.randomSeed.wrapping_add(enet_host_random_seed());
    host.randomSeed = (host.randomSeed << 16) | (host.randomSeed >> 16);

    let rc = Rc::new(RefCell::new(host));
    let mut host = rc.borrow_mut();

    let mut peers = Vec::with_capacity(peerCount);

    for i in 0..peerCount {
        peers.push(ENetPeer {
            host: rc.clone(),
            outgoingPeerID: 0,
            incomingPeerID: i as u16,
            connectID: 0,
            outgoingSessionID: 0xFF,
            incomingSessionID: 0xFF,
            address: ENetAddress::new(),
            data: None,
            state: ENetPeerState::ENET_PEER_STATE_DISCONNECTED,
            channels: vec![],
            channelCount: 0,
            incomingBandwidth: 0,
            outgoingBandwidth: 0,
            incomingBandwidthThrottleEpoch: 0,
            outgoingBandwidthThrottleEpoch: 0,
            incomingDataTotal: 0,
            outgoingDataTotal: 0,
            lastSendTime: 0,
            lastReceiveTime: 0,
            nextTimeout: 0,
            earliestTimeout: 0,
            packetLossEpoch: 0,
            packetsSent: 0,
            packetsLost: 0,
            packetLoss: 0,
            packetLossVariance: 0,
            packetThrottle: 0,
            packetThrottleLimit: 0,
            packetThrottleCounter: 0,
            packetThrottleEpoch: 0,
            packetThrottleAcceleration: 0,
            packetThrottleDeceleration: 0,
            packetThrottleInterval: 0,
            pingInterval: 0,
            timeoutLimit: 0,
            timeoutMinimum: 0,
            timeoutMaximum: 0,
            lastRoundTripTime: 0,
            lowestRoundTripTime: 0,
            lastRoundTripTimeVariance: 0,
            highestRoundTripTimeVariance: 0,
            roundTripTime: 0,
            roundTripTimeVariance: 0,
            mtu: 0,
            windowSize: 0,
            reliableDataInTransit: 0,
            outgoingReliableSequenceNumber: 0,
            acknowledgements: VecDeque::new(),
            sentReliableCommands: VecDeque::new(),
            outgoingSendReliableCommands: VecDeque::new(),
            outgoingCommands: VecDeque::new(),
            dispatchedCommands: VecDeque::new(),
            flags: 0,
            reserved: 0,
            incomingUnsequencedGroup: 0,
            outgoingUnsequencedGroup: 0,
            unsequencedWindow: [0u32; (ENET_PEER_UNSEQUENCED_WINDOW_SIZE / 32) as usize],
            eventData: 0,
            totalWaitingData: 0,
        });
    }

    host.peers = peers.into_boxed_slice();

    drop(host);
    Some(rc)
}

pub fn enet_host_destroy(host: ENetHost) {
    if let Some(context) = host.compressor.context {
        if let Some(destroy) = host.compressor.destroy {
            destroy(Some(context));
        }
    }
}

pub fn enet_host_random(host: &mut ENetHost) -> u32 {
    let mut n = host.randomSeed.wrapping_add(0x6D2B79F5);
    n = (n ^ (n >> 15)) * (n | 1);
    n ^= n + (n ^ (n >> 7)) * (n | 61);
    n ^ (n >> 14)
}
