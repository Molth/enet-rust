#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub const ENET_PROTOCOL_MINIMUM_MTU: u32 = 576;
pub const ENET_PROTOCOL_MAXIMUM_MTU: u32 = 4096;
pub const ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS: u32 = 32;
pub const ENET_PROTOCOL_MINIMUM_WINDOW_SIZE: u32 = 4096;
pub const ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE: u32 = 65536;
pub const ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT: u32 = 1;
pub const ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT: u32 = 255;
pub const ENET_PROTOCOL_MAXIMUM_PEER_ID: u32 = 0xFFF;
pub const ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT: u32 = 1024 * 1024;

#[repr(u32)]
pub enum ENetProtocolCommand {
    ENET_PROTOCOL_COMMAND_NONE = 0,
    ENET_PROTOCOL_COMMAND_ACKNOWLEDGE = 1,
    ENET_PROTOCOL_COMMAND_CONNECT = 2,
    ENET_PROTOCOL_COMMAND_VERIFY_CONNECT = 3,
    ENET_PROTOCOL_COMMAND_DISCONNECT = 4,
    ENET_PROTOCOL_COMMAND_PING = 5,
    ENET_PROTOCOL_COMMAND_SEND_RELIABLE = 6,
    ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE = 7,
    ENET_PROTOCOL_COMMAND_SEND_FRAGMENT = 8,
    ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED = 9,
    ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT = 10,
    ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE = 11,
    ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT = 12,
    ENET_PROTOCOL_COMMAND_COUNT = 13,

    ENET_PROTOCOL_COMMAND_MASK = 0x0F,
}

#[repr(u32)]
pub enum ENetProtocolFlag {
    ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE = 1 << 7,
    ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED = 1 << 6,
    ENET_PROTOCOL_HEADER_FLAG_COMPRESSED = 1 << 14,
    ENET_PROTOCOL_HEADER_FLAG_SENT_TIME = 1 << 15,

    ENET_PROTOCOL_HEADER_FLAG_MASK = ENetProtocolFlag::ENET_PROTOCOL_HEADER_FLAG_COMPRESSED as u32
        | ENetProtocolFlag::ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as u32,

    ENET_PROTOCOL_HEADER_SESSION_MASK = 3 << 12,
    ENET_PROTOCOL_HEADER_SESSION_SHIFT = 12,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolHeader {
    pub peerID: u16,
    pub sentTime: u16,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolCommandHeader {
    pub command: u8,
    pub channelID: u8,
    pub reliableSequenceNumber: u16,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolAcknowledge {
    pub header: ENetProtocolCommandHeader,
    pub receivedReliableSequenceNumber: u16,
    pub receivedSentTime: u16,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: u16,
    pub incomingSessionID: u8,
    pub outgoingSessionID: u8,
    pub mtu: u32,
    pub windowSize: u32,
    pub channelCount: u32,
    pub incomingBandwidth: u32,
    pub outgoingBandwidth: u32,
    pub packetThrottleInterval: u32,
    pub packetThrottleAcceleration: u32,
    pub packetThrottleDeceleration: u32,
    pub connectID: u32,
    pub data: u32,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolVerifyConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: u16,
    pub incomingSessionID: u8,
    pub outgoingSessionID: u8,
    pub mtu: u32,
    pub windowSize: u32,
    pub channelCount: u32,
    pub incomingBandwidth: u32,
    pub outgoingBandwidth: u32,
    pub packetThrottleInterval: u32,
    pub packetThrottleAcceleration: u32,
    pub packetThrottleDeceleration: u32,
    pub connectID: u32,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolBandwidthLimit {
    pub header: ENetProtocolCommandHeader,
    pub incomingBandwidth: u32,
    pub outgoingBandwidth: u32,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolThrottleConfigure {
    pub header: ENetProtocolCommandHeader,
    pub packetThrottleInterval: u32,
    pub packetThrottleAcceleration: u32,
    pub packetThrottleDeceleration: u32,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolDisconnect {
    pub header: ENetProtocolCommandHeader,
    pub data: u32,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolPing {
    pub header: ENetProtocolCommandHeader,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolSendReliable {
    pub header: ENetProtocolCommandHeader,
    pub dataLength: u16,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolSendUnreliable {
    pub header: ENetProtocolCommandHeader,
    pub unreliableSequenceNumber: u16,
    pub dataLength: u16,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolSendUnsequenced {
    pub header: ENetProtocolCommandHeader,
    pub unsequencedGroup: u16,
    pub dataLength: u16,
}

#[derive(Copy, Clone, Default)]
pub struct ENetProtocolSendFragment {
    pub header: ENetProtocolCommandHeader,
    pub startSequenceNumber: u16,
    pub dataLength: u16,
    pub fragmentCount: u32,
    pub fragmentNumber: u32,
    pub totalLength: u32,
    pub fragmentOffset: u32,
}

#[derive(Copy, Clone)]
pub enum ENetProtocol {
    header(ENetProtocolCommandHeader),
    acknowledge(ENetProtocolAcknowledge),
    connect(ENetProtocolConnect),
    verifyConnect(ENetProtocolVerifyConnect),
    disconnect(ENetProtocolDisconnect),
    ping(ENetProtocolPing),
    sendReliable(ENetProtocolSendReliable),
    sendUnreliable(ENetProtocolSendUnreliable),
    sendUnsequenced(ENetProtocolSendUnsequenced),
    sendFragment(ENetProtocolSendFragment),
    bandwidthLimit(ENetProtocolBandwidthLimit),
    throttleConfigure(ENetProtocolThrottleConfigure),
}

impl Default for ENetProtocol {
    fn default() -> Self {
        ENetProtocol::header(ENetProtocolCommandHeader::default())
    }
}
