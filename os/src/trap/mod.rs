mod context;

pub use context::TrapContext;

use crate::{syscall::syscall, time::set_next_trigger, task::suspend_current_and_run_next};
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.spec += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("[kernel] PageFault in application, kernel killed it.");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, kernel killed it.");
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => { set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!("Unsupport trap {:?}, stval = {:#x}", scause.cause(), stval);
        }
    }
    cx
}
