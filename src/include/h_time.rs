#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub const ENET_TIME_OVERFLOW: u32 = 86400000;

pub fn ENET_TIME_LESS(a: u32, b: u32) -> bool {
    a.wrapping_sub(b) >= ENET_TIME_OVERFLOW
}

pub fn ENET_TIME_GREATER(a: u32, b: u32) -> bool {
    b.wrapping_sub(a) >= ENET_TIME_OVERFLOW
}

pub fn ENET_TIME_LESS_EQUAL(a: u32, b: u32) -> bool {
    !ENET_TIME_GREATER(a, b)
}

pub fn ENET_TIME_GREATER_EQUAL(a: u32, b: u32) -> bool {
    !ENET_TIME_LESS(a, b)
}

pub fn ENET_TIME_DIFFERENCE(a: u32, b: u32) -> u32 {
    if a.wrapping_sub(b) >= ENET_TIME_OVERFLOW {
        b.wrapping_sub(a)
    } else {
        a.wrapping_sub(b)
    }
}