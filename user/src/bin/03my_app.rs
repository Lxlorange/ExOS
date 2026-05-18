#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("My Custom App: Sum of 1 to 100");
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    println!("The result is: {}", sum);
    println!("My App executed successfully!");
    0
}