#![feature(panic_info_message)]
#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]

use core::arch::global_asm;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod sbi;
mod memory;
mod sync;
pub mod trap;
pub mod syscall;
pub mod task;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

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

fn print_memory() {
    trap::init();
}

#[no_mangle]
fn rust_main() {
    clear_bss();
    print_memory();
    trap::init();
    loader::load_apps();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}
