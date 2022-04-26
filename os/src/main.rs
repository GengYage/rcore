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

fn print_memory() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
    }
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    error!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
}

#[no_mangle]
fn rust_main() {
    clear_bss();
    print_memory();
    panic!("Shutdown machine!");
}
