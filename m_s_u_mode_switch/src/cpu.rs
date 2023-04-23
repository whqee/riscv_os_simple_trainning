use core::arch::asm;

#[derive(Debug, Clone, Copy)]
pub enum MPP {
    User,
    Supervisor,
    Machine = 3,
}

#[allow(unused)]
pub enum SPP {
    User,
    Supervisor,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExceptionCause {
    Exception(Exception),
    Interrupt(Interrupt),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    BreakPoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAMOAddressMisaligned,
    StoreAMOAccessFault,
    EcallFromU,
    EcallFromS,
    // Reserved,
    EcallFromM = 11,
    InstructionPageFault,
    LoadPageFault,
    // Reserved,
    StoreAMOPageFault = 15,
    // Reserved...,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Interrupt {
    // Reserved,
    SSoftwareInt = 1,
    // Reserved,
    MSoftwareInt = 3,
    // Reserved,
    STimerInt = 5,
    // Reserved,
    MTimerInt = 7,
    // Reserved,
    SExternalInt = 9,
    // Reserved,
    MExternalInt = 11,
    // Reserved...,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MStatus {
    pub(crate) raw: usize,
}

pub struct Mcause {
    mcause: usize,
}

#[allow(unused)]
pub struct Sstatus;

#[allow(unused)]
pub struct Scause;

#[allow(unused)]
impl MPP {
    #[inline]
    pub fn set(mpp: MPP) {
        MStatus::write(MStatus::read().raw & !(3usize << 11) | ((mpp as usize) << 11))
    }

    #[inline]
    pub fn get() -> MPP {
        MStatus::read().into()
    }
}

#[allow(unused)]
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

macro_rules! read_csr {
    ($csr_number:literal) => {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            let r: usize;
            core::arch::asm!(concat!("csrrs {0}, ", stringify!($csr_number), ", x0"), out(reg) r);
            r
        }
    };
}

impl MStatus {
    // Create inline function for mstatus: _read() -> usize(mstatus)
    read_csr!(0x300);

    // #[inline]
    #[no_mangle]
    pub fn read() -> MStatus {
        // let raw: usize;
        // unsafe {
        //     asm!("csrr {0}, mstatus", out(reg)raw);
        // }
        // MStatus { raw }
        MStatus {
            raw: unsafe { Self::_read() },
        }
    }

    #[inline]
    pub fn write(mstatus: usize) {
        unsafe {
            asm!("csrw mstatus, {0}", in(reg)mstatus);
        }
    }

    #[allow(unused)]
    #[inline]
    pub fn raw(&self) -> usize {
        self.raw
    }
}

impl From<MStatus> for MPP {
    fn from(mstatus: MStatus) -> Self {
        match (mstatus.raw >> 11) & 3 {
            0 => MPP::User,
            1 => MPP::Supervisor,
            3 => MPP::Machine,
            _ => panic!("Unsupported 'MPP' code at 'mstatus'."),
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
    pub fn mcause() -> Self {
        let mcause: usize;
        unsafe {
            asm!("csrr {0}, mcause", out(reg)mcause);
        }
        Mcause { mcause }
    }
}

impl From<Mcause> for ExceptionCause {
    fn from(mc: Mcause) -> Self {
        // Exception or Interrupt ?
        if (mc.mcause >> (core::mem::size_of::<usize>() - 1)) == 0 {
            // Exception
            match mc.mcause {
                0 => ExceptionCause::Exception(Exception::InstructionAddressMisaligned),
                1 => ExceptionCause::Exception(Exception::InstructionAccessFault),
                2 => ExceptionCause::Exception(Exception::IllegalInstruction),
                3 => ExceptionCause::Exception(Exception::BreakPoint),
                4 => ExceptionCause::Exception(Exception::LoadAddressMisaligned),
                5 => ExceptionCause::Exception(Exception::LoadAccessFault),
                6 => ExceptionCause::Exception(Exception::StoreAMOAddressMisaligned),
                7 => ExceptionCause::Exception(Exception::StoreAMOAccessFault),
                8 => ExceptionCause::Exception(Exception::EcallFromU),
                9 => ExceptionCause::Exception(Exception::EcallFromS),
                11 => ExceptionCause::Exception(Exception::EcallFromM),
                12 => ExceptionCause::Exception(Exception::InstructionPageFault),
                13 => ExceptionCause::Exception(Exception::LoadPageFault),
                15 => ExceptionCause::Exception(Exception::StoreAMOPageFault),
                _ => todo!(),
            }
        } else {
            // Interrupt
            match mc.mcause & !(1 << (core::mem::size_of::<usize>() - 1)) {
                1 => ExceptionCause::Interrupt(Interrupt::SSoftwareInt),
                3 => ExceptionCause::Interrupt(Interrupt::MSoftwareInt),
                5 => ExceptionCause::Interrupt(Interrupt::STimerInt),
                7 => ExceptionCause::Interrupt(Interrupt::MTimerInt),
                9 => ExceptionCause::Interrupt(Interrupt::SExternalInt),
                11 => ExceptionCause::Interrupt(Interrupt::MExternalInt),
                _ => todo!(),
            }
        }
    }
}

#[allow(unused)]
#[inline]
fn mie() -> usize {
    let val: usize;
    unsafe {
        asm!("csrr {0}, mie", out(reg)val);
    }
    val
}

// #[inline]
// fn set_mie(mie: usize) {
//     unsafe {
//         asm!("csrw mie, {0}", in(reg)mie);
//     }
// }

#[inline]
pub(crate) fn enable_m_interrupt() {
    MStatus::write(MStatus::read().raw | (1 << 3))
}

// #[inline]
// pub fn enable_m_timer_interrupt() {
//     set_mie(mie() | (1 << 7))
// }

// #[inline]
// pub fn get_time() -> u64 {
//     crate::clint::CLint::get_mtime()
// }

// #[inline]
// pub fn set_timer(time_val: u64) {
//     enable_m_timer_interrupt();
//     crate::clint::CLint::set_timer(time_val);
// }

// #[inline]
// pub fn hart_id() -> usize {
//     let val: usize;
//     unsafe {
//         asm!("csrr {0}, hartid", out(reg)val);
//     }
//     val
// }

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

// #[inline]
// pub fn mip() -> usize {
//     let val: usize;
//     unsafe {
//         asm!("csrr {0}, mip", out(reg)val);
//     }
//     val
// }

// #[inline]
// pub fn set_mip(mip: usize) {
//     unsafe {
//         asm!("csrw mip, {0}", in(reg)mip);
//     }
// }

// #[inline]
// pub fn mhartid() -> usize {
//     let val: usize;
//     unsafe {
//         asm!("csrr {0}, mhartid", out(reg)val);
//     }
//     val
// }

// #[inline]
// pub fn medeleg() -> usize {
//     let val: usize;
//     unsafe {
//         asm!("csrr {0}, medeleg", out(reg)val);
//     }
//     val
// }

// #[inline]
// pub fn mideleg() -> usize {
//     let val: usize;
//     unsafe {
//         asm!("csrr {0}, mideleg", out(reg)val);
//     }
//     val
// }

#[inline]
pub fn mepc() -> usize {
    let val: usize;
    unsafe {
        asm!("csrr {0}, mepc", out(reg)val);
    }
    val
}

// #[inline]
// pub fn set_mepc(mepc: usize) {
//     unsafe {
//         asm!("csrw mepc, {0}", in(reg)mepc);
//     }
// }

#[macro_export]
macro_rules! mret {
    () => {{
        unsafe { core::arch::asm!("mret", options(noreturn)) }
    }};
}
