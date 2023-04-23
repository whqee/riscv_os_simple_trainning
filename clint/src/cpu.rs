use core::arch::asm;
pub enum MPP {
    User,
    Supervisor,
    Machine = 3,
}

pub enum SPP {
    User,
    Supervisor,
}

pub struct MStatus {
    mstatus: usize,
}
pub struct Mcause {
    mcause: usize,
}
pub struct Sstatus;
pub struct Scause;

impl MPP {
    #[inline]
    pub fn set(mpp: MPP) {
        MStatus::write(MStatus::read().mstatus & !(3usize << 11) | ((mpp as usize) << 11))
    }

    #[inline]
    pub fn get() -> MPP {
        MStatus::read().into()
    }
}

impl SPP {
    #[inline]
    pub fn get() -> SPP {
        match (Sstatus::read() >> 8) & 1 {
            0 => SPP::User,
            1 => SPP::Supervisor,
            _ => panic!("Unsupported 'SPP' code at 'sstatus'."),
        }
    }

    #[inline]
    pub fn set(spp: SPP) {
        Sstatus::write(Sstatus::read() & !(1usize << 8) | ((spp as usize) << 8))
    }
}

impl MStatus {
    #[inline]
    pub fn read() -> MStatus {
        let mstatus: usize;
        unsafe {
            asm!("csrr {0}, mstatus", out(reg)mstatus);
        }
        MStatus { mstatus }
    }

    #[inline]
    pub fn write(mstatus: usize) {
        unsafe {
            asm!("csrw mstatus, {0}", in(reg)mstatus);
        }
    }
}

impl Sstatus {
    #[inline]
    pub fn read() -> usize {
        let val: usize;
        unsafe {
            asm!("csrr {0}, sstatus", out(reg)val);
        }
        val
    }

    #[inline]
    pub fn write(sstatus: usize) {
        unsafe {
            asm!("csrw sstatus, {0}", in(reg)sstatus);
        }
    }
}

impl Mcause {
    #[inline]
    pub fn mcause() -> usize {
        let val: usize;
        unsafe {
            asm!("csrr {0}, mcause", out(reg)val);
        }
        val
    }
}

impl From<MStatus> for MPP {
    fn from(mstatus: MStatus) -> Self {
        match (mstatus.mstatus >> 11) & 3 {
            0 => MPP::User,
            1 => MPP::Supervisor,
            3 => MPP::Machine,
            _ => panic!("Unsupported 'MPP' code at 'mstatus'."),
        }
    }
}

#[inline]
fn mie() -> usize {
    let val: usize;
    unsafe {
        asm!("csrr {0}, mie", out(reg)val);
    }
    val
}

#[inline]
fn set_mie(mie: usize) {
    unsafe {
        asm!("csrw mie, {0}", in(reg)mie);
    }
}

#[inline]
pub(crate) fn enable_m_interrupt() {
    MStatus::write(MStatus::read().mstatus | (1 << 3))
}

#[inline]
pub fn enable_m_timer_interrupt() {
    set_mie(mie() | (1 << 7))
}

#[inline]
pub fn get_time() -> u64 {
    crate::clint::CLint::get_mtime()
}

#[inline]
pub fn set_timer(time_val: u64) {
    enable_m_timer_interrupt();
    crate::clint::CLint::set_timer(time_val);
}

#[inline]
pub fn hart_id() -> usize {
    let val: usize;
    unsafe {
        asm!("csrr {0}, hartid", out(reg)val);
    }
    val
}

#[inline]
pub fn set_mtvec_direct_mode(trap_handler: usize) {
    unsafe {
        asm!("csrw mtvec, {0}", in(reg)(trap_handler));
    }
}

#[inline]
pub fn set_mscratch(mscratch: usize) {
    unsafe {
        asm!("csrw mscratch, {0}", in(reg)(mscratch));
    }
}

#[inline]
pub fn mip() -> usize {
    let val: usize;
    unsafe {
        asm!("csrr {0}, mip", out(reg)val);
    }
    val
}

#[inline]
pub fn set_mip(mip: usize) {
    unsafe {
        asm!("csrw mip, {0}", in(reg)mip);
    }
}

#[inline]
pub fn mhartid() -> usize {
    let val: usize;
    unsafe {
        asm!("csrr {0}, mhartid", out(reg)val);
    }
    val
}

#[inline]
pub fn medeleg() -> usize {
    let val: usize;
    unsafe {
        asm!("csrr {0}, medeleg", out(reg)val);
    }
    val
}

#[inline]
pub fn mideleg() -> usize {
    let val: usize;
    unsafe {
        asm!("csrr {0}, mideleg", out(reg)val);
    }
    val
}
