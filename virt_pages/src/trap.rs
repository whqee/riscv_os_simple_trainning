use crate::{
    cpu,
    mmu::Sv39Satp,
    print, println,
    switch::{self, Context, RegFrame, CONTEXT_APP_HELLO, CONTEXT_M, CONTEXT_S, CONTEXT_U},
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
pub fn m_trap_handler(frame: &mut RegFrame) {
    // println!("mcause: 0x{:x}", cpu::Mcause::mcause());
    // println!("mip: 0x{:x}", cpu::mip());
    // cpu::set_mip(0);

    // Simple trace log
    let cause = cpu::Mcause::mcause().into();
    // let mpp: cpu::MPP = cx.frame.mstatus.into();
    print!(
        "[M][Trap] {:?}, mepc=0x{:x}, mepc=0x{:x} ",
        cause,
        cpu::mepc(),
        frame.mepc
    );

    let cx: &mut Context = frame.into();

    match cause {
        cpu::ExceptionCause::Exception(e) => match e {
            cpu::Exception::InstructionAddressMisaligned => todo!(),
            cpu::Exception::InstructionAccessFault => {
                println!("{:#X?}", frame);
                todo!()
            }
            cpu::Exception::IllegalInstruction => {
                println!("{:#X?}", frame);
                todo!()
            }
            cpu::Exception::BreakPoint => {
                cx.frame.clone_from(&frame);
                frame.clone_from(unsafe { &CONTEXT_M.frame });
            }
            cpu::Exception::LoadAddressMisaligned => todo!(),
            cpu::Exception::LoadAccessFault => {
                println!("{:#X?}", frame);
                todo!()
            }
            cpu::Exception::StoreAMOAddressMisaligned => todo!(),
            cpu::Exception::StoreAMOAccessFault => todo!(),
            cpu::Exception::ECallFromU => {
                frame.mepc += 4;
                // println!("{:#X?}", frame);
                match frame.a7.into() {
                    // switch context to (a0)
                    ECall::SwitchToContext => unsafe {
                        cx.frame.clone_from(&frame);
                        frame.clone_from(&*(frame.a0 as *mut RegFrame));
                        print!(
                            ", switching to context {{ mepc=0x{:x} ra=0x{:x} Mode={:?} satp:0x{:x} }}",
                            frame.mepc,
                            frame.ra,
                            cpu::MPP::from(frame.mstatus),
                            frame.satp
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
                        cx.frame.clone_from(&frame);
                        frame.clone_from(&*(frame.a0 as *mut RegFrame));
                        print!(
                            ", switching to context {{ mepc=0x{:x} ra=0x{:x} Mode={:?} satp:0x{:x} }}",
                            frame.mepc,
                            frame.ra,
                            cpu::MPP::from(frame.mstatus),
                            frame.satp
                        );
                    },
                    _ => todo!(),
                }
            }
            cpu::Exception::ECallFromM => {
                frame.mepc += 4;
                // println!("{:#X?}", frame);
                match frame.a7.into() {
                    // switch context to (a0)
                    ECall::SwitchToContext => unsafe {
                        cx.frame.clone_from(&frame);
                        frame.clone_from(&*(frame.a0 as *mut RegFrame));
                        print!(
                            ", switching to context {{ mepc=0x{:x} ra=0x{:x} Mode={:?} satp:0x{:x} }}",
                            frame.mepc,
                            frame.ra,
                            cpu::MPP::from(frame.mstatus),
                            frame.satp
                        );
                    },
                    _ => todo!(),
                }
            }
            cpu::Exception::InstructionPageFault => {
                println!("{:#X?}", frame);
                todo!()
            }
            cpu::Exception::LoadPageFault => {
                println!("{:#X?}", frame);
                todo!()
            }
            cpu::Exception::StoreAMOPageFault => {
                println!("{:#X?}", frame);
                todo!()
            }
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
            _ => todo!(),
        }
    }
}

impl From<&mut RegFrame> for &mut Context {
    fn from(f: &mut RegFrame) -> Self {
        let cx_list = unsafe {
            [
                // &mut CONTEXT_M,
                &mut CONTEXT_S,
                &mut CONTEXT_U,
                &mut CONTEXT_APP_HELLO,
            ]
        };

        if f.satp == 0 {
            return unsafe { &mut CONTEXT_M };
        }

        for i in cx_list.into_iter() {
            if Sv39Satp::from(i.root).usize() == f.satp {
                return i;
            }
        }

        panic!("Can't find a proccess who hold the satp. Frame = {:#x?}", f)

        // todo!()
    }
}
