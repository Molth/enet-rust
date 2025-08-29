#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub const fn ENET_MAX(x: u32, y: u32) -> u32 {
    if x > y {
        x
    } else {
        y
    }
}

pub const fn ENET_MIN(x: u32, y: u32) -> u32 {
    if x < y {
        x
    } else {
        y
    }
}

pub const fn ENET_DIFFERENCE(x: u32, y: u32) -> u32 {
    if x < y {
        y - x
    } else {
        x - y
    }
}
