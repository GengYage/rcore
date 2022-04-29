use riscv::register::sstatus::{self, SPP, Sstatus};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub spec: usize,
}

impl TrapContext {
    pub fn set_up(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = TrapContext {
            x: [0; 32],
            sstatus,
            spec: entry,
        };
        cx.set_up(sp);
        cx
    }
}