#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::yield_;

const WIDTH: usize = 10;
const HEIGHT: usize = 2;

#[no_mangle]
fn main() -> i32 {
    for i in 0..HEIGHT {
        print!("{}", "\x1b[93m");
        for _ in 0..WIDTH {
            print!("B");
        }
        print!(" [{}/{}]", i + 1, HEIGHT);
        print!("{}", "\n\x1b[0m");
        yield_();
    }
    warn!("Test write_b OK!");
    0
}
