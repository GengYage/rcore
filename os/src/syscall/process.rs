use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};

pub(crate) fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited whith code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unrechable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}
