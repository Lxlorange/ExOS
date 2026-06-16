#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::shutdown;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    shutdown();
    0
}
