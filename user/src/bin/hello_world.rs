#![no_std]
#![no_main]

use user_lib::println;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, world!");
    0
}
