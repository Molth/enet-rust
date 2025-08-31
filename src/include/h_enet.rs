#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::h_protocol::*;
use crate::h_win32::ENetBuffer;
use std::any::Any;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};
use std::rc::Rc;

pub const ENET_VERSION_MAJOR: u32 = 1;
pub const ENET_VERSION_MINOR: u32 = 3;
pub const ENET_VERSION_PATCH: u32 = 18;

pub const fn ENET_VERSION_CREATE(major: u32, minor: u32, patch: u32) -> u32 {
    ((major) << 16) | ((minor) << 8) | (patch)
}

pub const fn ENET_VERSION_GET_MAJOR(version: u32) -> u32 {
    ((version) >> 16) & 0xFF
}

pub const fn ENET_VERSION_GET_MINOR(version: u32) -> u32 {
    ((version) >> 8) & 0xFF
}

pub const fn ENET_VERSION_GET_PATCH(version: u32) -> u32 {
    (version) & 0xFF
}

pub const ENET_VERSION: u32 =
    ENET_VERSION_CREATE(ENET_VERSION_MAJOR, ENET_VERSION_MINOR, ENET_VERSION_PATCH);

#[repr(u32)]
pub enum ENetSocketType {
    ENET_SOCKET_TYPE_STREAM = 1,
    ENET_SOCKET_TYPE_DATAGRAM = 2,
}

#[repr(u32)]
pub enum ENetSocketWait {
    ENET_SOCKET_WAIT_NONE = 0,
    ENET_SOCKET_WAIT_SEND = 1 << 0,
    ENET_SOCKET_WAIT_RECEIVE = 1 << 1,
    ENET_SOCKET_WAIT_INTERRUPT = 1 << 2,
}

#[repr(u32)]
pub enum ENetSocketOption {
    ENET_SOCKOPT_NONBLOCK = 1,
    ENET_SOCKOPT_BROADCAST = 2,
    ENET_SOCKOPT_RCVBUF = 3,
    ENET_SOCKOPT_SNDBUF = 4,
    ENET_SOCKOPT_REUSEADDR = 5,
    ENET_SOCKOPT_RCVTIMEO = 6,
    ENET_SOCKOPT_SNDTIMEO = 7,
    ENET_SOCKOPT_ERROR = 8,
    ENET_SOCKOPT_NODELAY = 9,
    ENET_SOCKOPT_TTL = 10,
    ENET_SOCKOPT_IPV6_ONLY = 11,
}

#[repr(u32)]
pub enum ENetSocketShutdown {
    ENET_SOCKET_SHUTDOWN_READ = 0,
    ENET_SOCKET_SHUTDOWN_WRITE = 1,
    ENET_SOCKET_SHUTDOWN_READ_WRITE = 2,
}

pub const ENET_HOST_ANY: [u8; 16] = [0; 16];
pub const ENET_HOST_BROADCAST: [u8; 16] =
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255];
pub const ENET_PORT_ANY: u32 = 0;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ENetAddress {
    pub host: [u8; 16],
    pub port: u16,
    pub scopeID: u32,
}

impl ENetAddress {
    pub fn new() -> ENetAddress {
        ENetAddress {
            host: [0; 16],
            port: 0,
            scopeID: 0,
        }
    }

    pub fn is_ipv4(&self) -> bool {
        self.host[0..10].iter().all(|&b| b == 0) && self.host[10..12] == [0xFF, 0xFF]
    }

    pub fn is_ipv6(&self) -> bool {
        !self.is_ipv4()
    }

    pub fn try_into(&self, socket: UdpSocket) -> Option<SocketAddr> {
        match socket.local_addr() {
            Ok(addr) => {
                return if addr.is_ipv6() {
                    Some(self.into_ipv6())
                } else {
                    self.try_into_ipv4()
                }
            }

            Err(_) => {}
        }

        None
    }

    pub fn try_into_ipv4(&self) -> Option<SocketAddr> {
        if self.is_ipv4() {
            return Some(self.into_ipv4());
        }

        None
    }

    pub fn into_ipv4(self) -> SocketAddr {
        let bytes: [u8; 4] = self.host[12..16].try_into().unwrap();
        let ipv4 = Ipv4Addr::from(bytes);
        SocketAddr::V4(SocketAddrV4::new(ipv4, self.port))
    }

    pub fn into_ipv6(self) -> SocketAddr {
        let ipv6 = Ipv6Addr::from(self.host);
        SocketAddr::V6(SocketAddrV6::new(ipv6, self.port, 0, self.scopeID))
    }
}

impl From<SocketAddr> for ENetAddress {
    fn from(addr: SocketAddr) -> ENetAddress {
        let mut host = [0u8; 16];

        match addr {
            SocketAddr::V4(ipv4) => {
                host[10..12].copy_from_slice(&[0xFF, 0xFF]);
                host[12..16].copy_from_slice(&ipv4.ip().octets());
                ENetAddress {
                    host,
                    port: ipv4.port(),
                    scopeID: 0,
                }
            }

            SocketAddr::V6(ipv6) => {
                host.copy_from_slice(&ipv6.ip().octets());
                ENetAddress {
                    host,
                    port: ipv6.port(),
                    scopeID: ipv6.scope_id(),
                }
            }
        }
    }
}

impl Into<SocketAddr> for ENetAddress {
    fn into(self) -> SocketAddr {
        if self.is_ipv4() {
            self.into_ipv4()
        } else {
            self.into_ipv6()
        }
    }
}

#[repr(u32)]
pub enum ENetPacketFlag {
    ENET_PACKET_FLAG_RELIABLE = 1 << 0,
    ENET_PACKET_FLAG_UNSEQUENCED = 1 << 1,
    ENET_PACKET_FLAG_NO_ALLOCATE = 1 << 2,
    ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT = 1 << 3,
    ENET_PACKET_FLAG_SENT = 1 << 8,
}

#[derive(Default)]
pub struct ENetPacket {
    pub referenceCount: usize,
    pub flags: u32,
    pub data: Option<Rc<RefCell<Box<[u8]>>>>,
    pub dataLength: usize,
    pub freeCallback: Option<fn(ENetPacket)>,
    pub userData: Option<fn(Box<dyn Any>)>,
}

pub struct ENetAcknowledgement {
    pub sentTime: u32,
    pub command: ENetProtocol,
}

pub struct ENetOutgoingCommand {
    pub reliableSequenceNumber: u16,
    pub unreliableSequenceNumber: u16,
    pub sentTime: u32,
    pub roundTripTimeout: u32,
    pub queueTime: u32,
    pub fragmentOffset: u32,
    pub fragmentLength: u16,
    pub sendAttempts: u16,
    pub command: ENetProtocol,
    pub packet: ENetPacket,
}

pub struct ENetIncomingCommand {
    pub reliableSequenceNumber: u16,
    pub unreliableSequenceNumber: u16,
    pub command: ENetProtocol,
    pub fragmentCount: u32,
    pub fragmentsRemaining: u32,
    pub fragments: VecDeque<u32>,
    pub packet: ENetPacket,
}

#[repr(u32)]
pub enum ENetPeerState {
    ENET_PEER_STATE_DISCONNECTED = 0,
    ENET_PEER_STATE_CONNECTING = 1,
    ENET_PEER_STATE_ACKNOWLEDGING_CONNECT = 2,
    ENET_PEER_STATE_CONNECTION_PENDING = 3,
    ENET_PEER_STATE_CONNECTION_SUCCEEDED = 4,
    ENET_PEER_STATE_CONNECTED = 5,
    ENET_PEER_STATE_DISCONNECT_LATER = 6,
    ENET_PEER_STATE_DISCONNECTING = 7,
    ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT = 8,
    ENET_PEER_STATE_ZOMBIE = 9,
}

pub const ENET_BUFFER_MAXIMUM: u32 = 1 + 2 * ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS;

pub const ENET_HOST_RECEIVE_BUFFER_SIZE: u32 = 256 * 1024;
pub const ENET_HOST_SEND_BUFFER_SIZE: u32 = 256 * 1024;
pub const ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL: u32 = 1000;
pub const ENET_HOST_DEFAULT_MTU: u32 = 1392;
pub const ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE: u32 = 32 * 1024 * 1024;
pub const ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA: u32 = 32 * 1024 * 1024;
pub const ENET_PEER_DEFAULT_ROUND_TRIP_TIME: u32 = 500;
pub const ENET_PEER_DEFAULT_PACKET_THROTTLE: u32 = 32;
pub const ENET_PEER_PACKET_THROTTLE_SCALE: u32 = 32;
pub const ENET_PEER_PACKET_THROTTLE_COUNTER: u32 = 7;
pub const ENET_PEER_PACKET_THROTTLE_ACCELERATION: u32 = 2;
pub const ENET_PEER_PACKET_THROTTLE_DECELERATION: u32 = 2;
pub const ENET_PEER_PACKET_THROTTLE_INTERVAL: u32 = 5000;
pub const ENET_PEER_PACKET_LOSS_SCALE: u32 = 1 << 16;
pub const ENET_PEER_PACKET_LOSS_INTERVAL: u32 = 10000;
pub const ENET_PEER_WINDOW_SIZE_SCALE: u32 = 64 * 1024;
pub const ENET_PEER_TIMEOUT_LIMIT: u32 = 32;
pub const ENET_PEER_TIMEOUT_MINIMUM: u32 = 5000;
pub const ENET_PEER_TIMEOUT_MAXIMUM: u32 = 30000;
pub const ENET_PEER_PING_INTERVAL: u32 = 500;
pub const ENET_PEER_UNSEQUENCED_WINDOWS: u32 = 64;
pub const ENET_PEER_UNSEQUENCED_WINDOW_SIZE: u32 = 1024;
pub const ENET_PEER_FREE_UNSEQUENCED_WINDOWS: u32 = 32;
pub const ENET_PEER_RELIABLE_WINDOWS: u32 = 16;
pub const ENET_PEER_RELIABLE_WINDOW_SIZE: u32 = 0x1000;
pub const ENET_PEER_FREE_RELIABLE_WINDOWS: u32 = 8;

pub struct ENetChannel {
    pub outgoingReliableSequenceNumber: u16,
    pub outgoingUnreliableSequenceNumber: u16,
    pub usedReliableWindows: u16,
    pub reliableWindows: [u16; ENET_PEER_RELIABLE_WINDOWS as usize],
    pub incomingReliableSequenceNumber: u16,
    pub incomingUnreliableSequenceNumber: u16,
    pub incomingReliableCommands: Vec<ENetIncomingCommand>,
    pub incomingUnreliableCommands: Vec<ENetIncomingCommand>,
}

#[repr(u32)]
pub enum ENetPeerFlag {
    ENET_PEER_FLAG_NEEDS_DISPATCH = 1 << 0,
    ENET_PEER_FLAG_CONTINUE_SENDING = 1 << 1,
}

pub struct ENetPeer {
    pub outgoingPeerID: u16,
    pub incomingPeerID: u16,
    pub connectID: u32,
    pub outgoingSessionID: u8,
    pub incomingSessionID: u8,
    pub address: ENetAddress,
    pub data: Option<Box<dyn Any>>,
    pub state: ENetPeerState,
    pub channels: Vec<ENetChannel>,
    pub channelCount: usize,
    pub incomingBandwidth: u32,
    pub outgoingBandwidth: u32,
    pub incomingBandwidthThrottleEpoch: u32,
    pub outgoingBandwidthThrottleEpoch: u32,
    pub incomingDataTotal: u32,
    pub outgoingDataTotal: u32,
    pub lastSendTime: u32,
    pub lastReceiveTime: u32,
    pub nextTimeout: u32,
    pub earliestTimeout: u32,
    pub packetLossEpoch: u32,
    pub packetsSent: u32,
    pub packetsLost: u32,
    pub packetLoss: u32,
    pub packetLossVariance: u32,
    pub packetThrottle: u32,
    pub packetThrottleLimit: u32,
    pub packetThrottleCounter: u32,
    pub packetThrottleEpoch: u32,
    pub packetThrottleAcceleration: u32,
    pub packetThrottleDeceleration: u32,
    pub packetThrottleInterval: u32,
    pub pingInterval: u32,
    pub timeoutLimit: u32,
    pub timeoutMinimum: u32,
    pub timeoutMaximum: u32,
    pub lastRoundTripTime: u32,
    pub lowestRoundTripTime: u32,
    pub lastRoundTripTimeVariance: u32,
    pub highestRoundTripTimeVariance: u32,
    pub roundTripTime: u32,
    pub roundTripTimeVariance: u32,
    pub mtu: u32,
    pub windowSize: u32,
    pub reliableDataInTransit: u32,
    pub outgoingReliableSequenceNumber: u16,
    pub acknowledgements: VecDeque<ENetAcknowledgement>,
    pub sentReliableCommands: VecDeque<ENetOutgoingCommand>,
    pub outgoingSendReliableCommands: VecDeque<ENetOutgoingCommand>,
    pub outgoingCommands: VecDeque<ENetOutgoingCommand>,
    pub dispatchedCommands: VecDeque<ENetIncomingCommand>,
    pub flags: u16,
    pub reserved: u16,
    pub incomingUnsequencedGroup: u16,
    pub outgoingUnsequencedGroup: u16,
    pub unsequencedWindow: [u32; (ENET_PEER_UNSEQUENCED_WINDOW_SIZE / 32) as usize],
    pub eventData: u32,
    pub totalWaitingData: usize,
}

pub struct ENetCompressor {
    pub context: Option<Box<dyn Any>>,
    pub compress: Option<
        fn(Option<Box<dyn Any>>, &mut [ENetBuffer], usize, usize, &mut [u8], usize) -> usize,
    >,
    pub decompress: Option<fn(Option<&Box<dyn Any>>, &[u8], usize, &mut [u8], usize) -> usize>,
    pub destroy: Option<fn(Option<Box<dyn Any>>)>,
}

impl ENetCompressor {
    pub fn new() -> ENetCompressor {
        ENetCompressor {
            context: None,
            compress: None,
            decompress: None,
            destroy: None,
        }
    }
}

pub struct ENetHost {
    pub socket: UdpSocket,
    pub address: ENetAddress,
    pub incomingBandwidth: u32,
    pub outgoingBandwidth: u32,
    pub bandwidthThrottleEpoch: u32,
    pub mtu: u32,
    pub randomSeed: u32,
    pub recalculateBandwidthLimits: i32,
    pub peers: Box<[ENetPeer]>,
    pub channelLimit: usize,
    pub serviceTime: u32,
    pub dispatchQueue: VecDeque<u16>,
    pub totalQueued: u32,
    pub packetSize: usize,
    pub headerFlags: u16,
    pub commands: [ENetProtocol; ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS as usize],
    pub commandCount: usize,
    pub buffers: [ENetBuffer; ENET_BUFFER_MAXIMUM as usize],
    pub bufferCount: usize,
    pub checksum: Option<fn(&mut [ENetBuffer], usize) -> u32>,
    pub compressor: ENetCompressor,
    pub packetData: [[u8; ENET_PROTOCOL_MAXIMUM_MTU as usize]; 2],
    pub receivedAddress: ENetAddress,
    pub receivedData: usize,
    pub receivedDataLength: usize,
    pub totalSentData: u32,
    pub totalSentPackets: u32,
    pub totalReceivedData: u32,
    pub totalReceivedPackets: u32,
    pub intercept: Option<fn(&mut ENetHost, &mut ENetEvent) -> i32>,
    pub connectedPeers: usize,
    pub bandwidthLimitedPeers: usize,
    pub duplicatePeers: usize,
    pub maximumPacketSize: usize,
    pub maximumWaitingData: usize,
}

impl ENetHost {
    pub fn get_peer(&self, incomingPeerID: u16) -> &ENetPeer {
        &self.peers[incomingPeerID as usize]
    }

    pub fn get_mut_peer(&mut self, incomingPeerID: u16) -> &mut ENetPeer {
        &mut self.peers[incomingPeerID as usize]
    }
}

#[repr(u32)]
pub enum ENetEventType {
    ENET_EVENT_TYPE_NONE = 0,
    ENET_EVENT_TYPE_CONNECT = 1,
    ENET_EVENT_TYPE_DISCONNECT = 2,
    ENET_EVENT_TYPE_RECEIVE = 3,
}

pub enum ENetEvent {
    NONE {
        peer: u16,
        channelID: u8,
        data: u32,
    },

    CONNECT {
        peer: u16,
        channelID: u8,
        data: u32,
    },

    DISCONNECT {
        peer: u16,
        channelID: u8,
        data: u32,
    },

    RECEIVE {
        peer: u16,
        channelID: u8,
        data: u32,
        packet: ENetPacket,
    },
}
