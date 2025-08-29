#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::time::Instant;

pub fn timeGetTime() -> i64 {
    let now = Instant::now();
    now.elapsed().as_millis() as i64
}
