#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exit, get_time, yield_};

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("hello_vm: start");
    for i in 0..5 {
        println!("hello_vm: round {}, time = {}", i, get_time());
        yield_();
    }
    println!("hello_vm: exit");
    exit(0);
    0
}