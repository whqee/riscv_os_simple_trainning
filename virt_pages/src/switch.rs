use core::{arch::asm, ptr::null_mut};

use crate::{cpu, mmu::PageTable, print};

// use alloc::boxed::Box;

// core::arch::global_asm!(include_str!("switch.S"));

// pub static mut SIMPLE_CONTEXT_LIST: [usize; 3] = [0usize; 3];
pub static mut CONTEXT_M: Context = crate::new_empty_context!();
pub static mut CONTEXT_S: Context = crate::new_empty_context!();
pub static mut CONTEXT_U: Context = crate::new_empty_context!();
pub static mut CONTEXT_APP_HELLO: Context = crate::new_empty_context!();

#[repr(C)]
#[derive(Clone)]
pub struct Context {
    pub frame: RegFrame,
    pub root: *mut PageTable,
    pub stack: *mut u8,
    pub mem: *mut u8,
    pub program_start: usize,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RegFrame {
    pub ra: usize,  // x1
    pub sp: usize,  // x2
    pub gp: usize,  // x3
    pub tp: usize,  // x4  is not needed here
    pub t0: usize,  // x5
    pub t1: usize,  // x6
    pub t2: usize,  // x7
    pub s0: usize,  // x8
    pub s1: usize,  // x9
    pub a0: usize,  // x10
    pub a1: usize,  // x11
    pub a2: usize,  // x12
    pub a3: usize,  // x13
    pub a4: usize,  // x14
    pub a5: usize,  // x15
    pub a6: usize,  // x16
    pub a7: usize,  // x17
    pub s2: usize,  // x18
    pub s3: usize,  // x19
    pub s4: usize,  // x20
    pub s5: usize,  // x21
    pub s6: usize,  // x22
    pub s7: usize,  // x23
    pub s8: usize,  // x24
    pub s9: usize,  // x25
    pub s10: usize, // x26
    pub s11: usize, // x27
    pub t3: usize,  // x28
    pub t4: usize,  // x29
    pub t5: usize,  // x30
    pub t6: usize,  // x31
    pub mepc: usize,
    // xscratch: usize,
    pub mstatus: cpu::MStatus,
    pub satp: usize,
}

impl core::fmt::Debug for Context {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        print!(
            "
            Context {{
                frame: RegFrame {{
                    pub ra: 0x{:x},  // x1
                    pub sp: 0x{:x},  // x2
                    pub gp: 0x{:x},  // x3
                    pub tp: 0x{:x},  // x4  is not needed here
                    pub t0: 0x{:x},  // x5
                    pub t1: 0x{:x},  // x6
                    pub t2: 0x{:x},  // x7
                    pub s0: 0x{:x},  // x8
                    pub s1: 0x{:x},  // x9
                    pub a0: 0x{:x},  // x10
                    pub a1: 0x{:x},  // x11
                    pub a2: 0x{:x},  // x12
                    pub a3: 0x{:x},  // x13
                    pub a4: 0x{:x},  // x14
                    pub a5: 0x{:x},  // x15
                    pub a6: 0x{:x},  // x16
                    pub a7: 0x{:x},  // x17
                    pub s2: 0x{:x},  // x18
                    pub s3: 0x{:x},  // x19
                    pub s4: 0x{:x},  // x20
                    pub s5: 0x{:x},  // x21
                    pub s6: 0x{:x},  // x22
                    pub s7: 0x{:x},  // x23
                    pub s8: 0x{:x},  // x24
                    pub s9: 0x{:x},  // x25
                    pub s10:0x{:x}, // x26
                    pub s11:0x{:x}, // x27
                    pub t3: 0x{:x},  // x28
                    pub t4: 0x{:x},  // x29
                    pub t5: 0x{:x},  // x30
                    pub t6: 0x{:x},  // x31
                    pub mepc: 0x{:x},
                    pub mstatus: cpu::MStatus {{
                        pub raw: 0x{:x}
                    }},
                    pub satp: 0x{:x},
                }},
                pub root: 0x{:x},
                pub stack: 0x{:x},
            }}
        ",
            self.frame.ra,
            self.frame.sp,
            self.frame.gp,
            self.frame.tp,
            self.frame.t0,
            self.frame.t1,
            self.frame.t2,
            self.frame.s0,
            self.frame.s1,
            self.frame.a0,
            self.frame.a1,
            self.frame.a2,
            self.frame.a3,
            self.frame.a4,
            self.frame.a5,
            self.frame.a6,
            self.frame.a7,
            self.frame.s2,
            self.frame.s3,
            self.frame.s4,
            self.frame.s5,
            self.frame.s6,
            self.frame.s7,
            self.frame.s8,
            self.frame.s9,
            self.frame.s10,
            self.frame.s11,
            self.frame.t3,
            self.frame.t4,
            self.frame.t5,
            self.frame.t6,
            self.frame.mepc,
            self.frame.mstatus.raw(),
            self.frame.satp,
            self.root as usize,
            self.stack as usize,
        );
        Ok(())
    }
}

// #[allow(unused)]
// #[inline(always)]
pub fn ecall_switch_to_context(cx: &mut Context) {
    let a0_param = 0x01usize; // Anyway, set '0x01' to be the ...
                              // ecall(1, cx as *mut Context as usize);
    unsafe {
        asm!("ecall", in("a7")a0_param, in("a0")cx);
    }
}

// #[no_mangle]
// fn ecall(_a0: usize, _a1: usize) {
//     unsafe { asm!("ecall") }
// }

// pub fn pending_context_list_push(cx: Context) {

// }

// pub fn pending_context_list_pop(cx: Context) -> Context {

//     todo!()
// }

#[allow(unused)]
#[inline]
pub fn new_simple_context(sp: usize, cx_entry: usize, mpp: cpu::MPP, satp: usize) -> Context {
    Context {
        frame: RegFrame {
            ra: 0,
            sp,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            mepc: cx_entry,
            mstatus: cpu::MStatus {
                raw: cpu::MStatus::read().raw & !(3usize << 11) | ((mpp as usize) << 11),
            },
            satp,
        },
        root: 0 as *mut PageTable,
        stack: 0 as *mut u8,
        mem: 0 as *mut u8,
        program_start: 0,
    }
}

#[macro_export]
/// new_simple_context!(sp: usize, cx_entry: usize, mpp: cpu::MPP)
macro_rules! new_empty_context {
    () => {
        Context {
            frame: crate::switch::RegFrame {
                ra: 0,
                sp: 0,
                gp: 0,
                tp: 0,
                t0: 0,
                t1: 0,
                t2: 0,
                s0: 0,
                s1: 0,
                a0: 0,
                a1: 0,
                a2: 0,
                a3: 0,
                a4: 0,
                a5: 0,
                a6: 0,
                a7: 0,
                s2: 0,
                s3: 0,
                s4: 0,
                s5: 0,
                s6: 0,
                s7: 0,
                s8: 0,
                s9: 0,
                s10: 0,
                s11: 0,
                t3: 0,
                t4: 0,
                t5: 0,
                t6: 0,
                mepc: 0,
                mstatus: cpu::MStatus { raw: 0 },
                satp: 0,
            },
            root: 0 as *mut PageTable,
            stack: 0 as *mut u8,
            mem: 0 as *mut u8,
            program_start: 0,
        }
    };
}
