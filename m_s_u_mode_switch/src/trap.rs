use crate::{
    cpu, print,
    switch::{self, Context},
};

core::arch::global_asm!(include_str!("trap.S"));

#[inline]
pub fn init() {
    extern "Rust" {
        fn __m_trap_entry();
    }
    cpu::set_mtvec_direct_mode(__m_trap_entry as usize);
}

#[no_mangle]
// Called by trap.S: __m_trap_entry
pub fn m_trap_handler(cx: &mut Context) {
    // println!("mcause: 0x{:x}", cpu::Mcause::mcause());
    // println!("mip: 0x{:x}", cpu::mip());
    // cpu::set_mip(0);

    // Simple trace log
    let cause = cpu::Mcause::mcause().into();
    // let mpp: cpu::MPP = cx.mstatus.into();
    print!("[M][Trap] {:?}, mepc=0x{:x}", cause, cpu::mepc());

    match cause {
        cpu::ExceptionCause::Exception(e) => match e {
            cpu::Exception::InstructionAddressMisaligned => todo!(),
            cpu::Exception::InstructionAccessFault => todo!(),
            cpu::Exception::IllegalInstruction => todo!(),
            cpu::Exception::BreakPoint => todo!(),
            cpu::Exception::LoadAddressMisaligned => todo!(),
            cpu::Exception::LoadAccessFault => todo!(),
            cpu::Exception::StoreAMOAddressMisaligned => todo!(),
            cpu::Exception::StoreAMOAccessFault => todo!(),
            cpu::Exception::ECallFromU => {
                cx.frame.mepc += 4;
                match cx.frame.a7.into() {
                    // switch context to (a0)
                    ECall::SwitchToContext => unsafe {
                        switch::CONTEXT_U.clone_from(cx);
                        cx.clone_from(&mut *(cx.frame.a0 as *mut Context));
                        print!(
                            ", switching to context {{ mepc=0x{:x} ra=0x{:x} Mode={:?} satp:0x{:x} }}",
                            cx.frame.mepc,
                            cx.frame.ra,
                            cpu::MPP::from(cx.mstatus),
                            cx.satp
                        );
                    },
                    _ => todo!(),
                }
            }
            cpu::Exception::ECallFromS => {
                cx.frame.mepc += 4;
                match cx.frame.a7.into() {
                    // switch context to (a0)
                    ECall::SwitchToContext => unsafe {
                        switch::CONTEXT_S.clone_from(cx);
                        cx.clone_from(&mut *(cx.frame.a0 as *mut Context));
                        print!(
                            ", switching to context {{ mepc=0x{:x} ra=0x{:x} Mode={:?} satp:0x{:x} }}",
                            cx.frame.mepc,
                            cx.frame.ra,
                            cpu::MPP::from(cx.mstatus),
                            cx.satp
                        );
                    },
                    _ => todo!(),
                }
            }
            cpu::Exception::ECallFromM => {
                cx.frame.mepc += 4;
                // println!("{:#?}", cx);
                match cx.frame.a7.into() {
                    // switch context to (a0)
                    ECall::SwitchToContext => unsafe {
                        switch::CONTEXT_M.clone_from(cx);
                        cx.clone_from(&mut *(cx.frame.a0 as *mut Context));
                        print!(
                            ", switching to context {{ mepc=0x{:x} ra=0x{:x} Mode={:?} satp:0x{:x} }}",
                            cx.frame.mepc,
                            cx.frame.ra,
                            cpu::MPP::from(cx.mstatus),
                            cx.satp
                        );
                    },
                    _ => todo!(),
                }
            }
            cpu::Exception::InstructionPageFault => todo!(),
            cpu::Exception::LoadPageFault => todo!(),
            cpu::Exception::StoreAMOPageFault => todo!(),
        },
        cpu::ExceptionCause::Interrupt(_) => todo!(),
    }

    static mut COUNT: i32 = 0;
    unsafe {
        COUNT = COUNT + 1;
        if COUNT > 9 {
            todo!()
        }
    }

    print!("\r\n");
    // return to trap.S
}

// fn exception_handler(e: cpu::Exception) {
//     match e {
//         cpu::Exception::InstructionAddressMisaligned => todo!(),
//         cpu::Exception::InstructionAccessFault => todo!(),
//         cpu::Exception::IllegalInstruction => todo!(),
//         cpu::Exception::BreakPoint => todo!(),
//         cpu::Exception::LoadAddressMisaligned => todo!(),
//         cpu::Exception::LoadAccessFault => todo!(),
//         cpu::Exception::StoreAMOAddressMisaligned => todo!(),
//         cpu::Exception::StoreAMOAccessFault => todo!(),
//         cpu::Exception::ECallFromS => todo!(),
//         cpu::Exception::ECallFromU => todo!(),
//         cpu::Exception::ECallFromM => todo!(),
//         cpu::Exception::InstructionPageFault => todo!(),
//         cpu::Exception::LoadPageFault => todo!(),
//         cpu::Exception::StoreAMOPageFault => todo!(),
//     }
// }

#[allow(unused)]
enum ECall {
    None,
    SwitchToContext,
}

impl From<usize> for ECall {
    fn from(val: usize) -> Self {
        match val {
            1 => ECall::SwitchToContext,
            _ => todo!()
        }
    }
}