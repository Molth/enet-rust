#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::h_enet::ENetSocketWait::*;
use crate::h_enet::{ENetAddress, ENetHostOption, ENetSocketOption};
use crate::h_system::timeGetTime;
use crate::h_win32::ENetBuffer;
use crate::{enet_buffer_as_mut_slice, enet_buffer_as_slice};
use std::io::Error;
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicU32, Ordering};

pub const SOCKET_ERROR: i32 = -1;

pub static timeBase: AtomicU32 = AtomicU32::new(0);

pub fn enet_initialize() {}

pub fn enet_deinitialize() {}

pub fn enet_host_random_seed() -> u32 {
    timeGetTime() as u32
}

pub fn enet_time_get() -> u32 {
    (timeGetTime() as u32) - timeBase.load(Ordering::SeqCst)
}

pub fn enet_time_set(newTimeBase: u32) {
    timeBase.store((timeGetTime() as u32) - newTimeBase, Ordering::SeqCst);
}

pub fn enet_socket_get_address(socket: &UdpSocket, address: &mut ENetAddress) -> Result<(), Error> {
    match socket.local_addr() {
        Ok(addr) => {
            *address = ENetAddress::from(addr);
            Ok(())
        }

        Err(e) => Err(e),
    }
}

pub fn enet_socket_create(
    address: &ENetAddress,
    option: ENetHostOption,
) -> Result<UdpSocket, Error> {
    match option {
        ENetHostOption::ENET_HOSTOPT_IPV4 => {
            UdpSocket::bind::<SocketAddr>((*address).try_parse_ipv4()?)
        }

        ENetHostOption::ENET_HOSTOPT_IPV6_ONLY | ENetHostOption::ENET_HOSTOPT_IPV6_DUALMODE => {
            UdpSocket::bind::<SocketAddr>((*address).parse_ipv6())
        }
    }
}

pub fn enet_socket_set_option(
    socket: &UdpSocket,
    option: ENetSocketOption,
    value: i32,
) -> Result<(), Error> {
    match option {
        ENetSocketOption::ENET_SOCKOPT_NONBLOCK => {
            socket.set_nonblocking(value != 0)?;
        }

        ENetSocketOption::ENET_SOCKOPT_BROADCAST => {
            socket.set_broadcast(value != 0)?;
        }

        _ => {}
    }

    Ok(())
}

pub fn enet_socket_destroy(socket: UdpSocket) {
    drop(socket);
}

pub fn enet_socket_send(
    socket: &UdpSocket,
    address: &ENetAddress,
    data: &[&[u8]],
    buffers: &[ENetBuffer],
    bufferCount: usize,
) -> i32 {
    match bufferCount {
        0 => 0,

        1 => {
            let buffer = enet_buffer_as_slice!(buffers[0], data);

            let result = address.try_parse_any_by_socket(socket);
            let socketAddr = match result {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            };

            match socket.send_to::<SocketAddr>(buffer, socketAddr) {
                Ok(len) => len as i32,
                Err(_) => -1,
            }
        }

        _ => {
            let merged: Vec<u8> = buffers
                .iter()
                .take(bufferCount)
                .flat_map(|buf| enet_buffer_as_slice!(buf, data))
                .copied()
                .collect();

            let result = address.try_parse_any_by_socket(socket);
            let socketAddr = match result {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            };

            match socket.send_to::<SocketAddr>(&merged, socketAddr) {
                Ok(len) => len as i32,
                Err(_) => -1,
            }
        }
    }
}

pub fn enet_socket_receive<'a>(
    socket: &UdpSocket,
    address: &mut ENetAddress,
    data: &'a mut [&'a mut [u8]],
    buffers: &mut [ENetBuffer],
    bufferCount: usize,
) -> i32 {
    match bufferCount {
        0 => 0,

        1 => {
            let buffer = enet_buffer_as_mut_slice!(buffers[0], data);
            match socket.recv_from(&mut *buffer) {
                Ok((len, addr)) => {
                    *address = ENetAddress::from(addr);
                    len as i32
                }

                Err(_) => -1,
            }
        }

        _ => {
            let total_len: usize = buffers
                .iter()
                .take(bufferCount)
                .map(|buf| buf.dataLength)
                .sum();

            let mut merged = vec![0; total_len];

            match socket.recv_from(&mut merged) {
                Ok((len, addr)) => {
                    *address = ENetAddress::from(addr);
                    let mut offset = 0;
                    for buf in buffers.iter_mut().take(bufferCount) {
                        let buffer = enet_buffer_as_mut_slice!(buf, data);
                        let buf_len = buffer.len();
                        if offset + buf_len <= len {
                            buffer.copy_from_slice(&merged[offset..offset + buf_len]);
                            offset += buf_len;
                        } else {
                            let remaining = len - offset;
                            if remaining > 0 {
                                buffer[..remaining]
                                    .copy_from_slice(&merged[offset..offset + remaining]);
                            }

                            break;
                        }
                    }

                    return len as i32;
                }

                Err(_) => -1,
            };

            -1
        }
    }
}

pub fn enet_socket_wait(socket: &UdpSocket, condition: &mut u32) -> bool {
    if *condition & (ENET_SOCKET_WAIT_SEND as u32) != 0 {
        *condition = ENET_SOCKET_WAIT_NONE as u32;
        *condition |= ENET_SOCKET_WAIT_SEND as u32;
        return true;
    }

    if *condition & (ENET_SOCKET_WAIT_RECEIVE as u32) != 0 {
        let mut buf = [0u8; 1];
        return match socket.peek(&mut buf) {
            Ok(_) => {
                *condition = ENET_SOCKET_WAIT_NONE as u32;
                *condition |= ENET_SOCKET_WAIT_RECEIVE as u32;
                true
            }

            Err(_) => false,
        };
    }

    true
}

pub fn enet_address_get_host_ip(address: &ENetAddress, ip: &mut [u8]) -> bool {
    let addr: SocketAddr = address.parse_any();
    let bytes = addr.ip().to_string().into_bytes();

    if bytes.len() > ip.len() {
        return false;
    }

    ip[..bytes.len()].copy_from_slice(&bytes);
    true
}

pub fn enet_address_get_host(address: &ENetAddress, hostName: &mut [u8]) -> bool {
    let addr: SocketAddr = address.parse_any();
    let bytes = addr.ip().to_string().into_bytes();

    if bytes.len() > hostName.len() {
        return false;
    }

    hostName[..bytes.len()].copy_from_slice(&bytes);
    true
}
