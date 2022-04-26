#![no_std]
#![no_main]

mod lang_items;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() {
    loop {}
}