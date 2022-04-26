#![feature(panic_info_message)]
#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]

use core::arch::global_asm;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod memory;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|byte| {
        unsafe {
            (byte as *mut u8).write_volatile(0);
        }
    });
}

#[no_mangle]
fn rust_main() {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}