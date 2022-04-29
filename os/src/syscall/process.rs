#![allow(dead_code)]
use crate::batch::run_next_app;

pub(crate) fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited whith code {}", exit_code);
    run_next_app()
}