use core::arch::asm;

use crate::{
    mmu::{PTEBit, PageTable, PTE},
    println,
};

// static S_STACK: [usize; 0x2000] = [0usize; 0x2000];

#[no_mangle]
pub fn smain() -> ! {
    println!("[S][Info] Hello !");
    println!("[S][Info] hello ! sstatus={:x}", read_sstatus() >> 8 & 0x1);
    // set_spp(SPP::User);
    // set_sepc(umode::umain as usize);
    // sret();

    let root = 0x8020_0000 as *mut PageTable;

    // 0 - 0x8000_0000 2GB space & 0x8... self-mapping
    // 0x4000_0000 = 0b .. 1 000000000(9bit) 000000000(9bit) _ 0000 0000 0000(12bit 4K) = 1 << 30
    // 0x8000_0000 = 1 << 31 = 0b10 << 30
    unsafe {
        (*root).entrys[0] = PTE {
            entry: 0 | PTEBit::RWX as usize | PTEBit::Valid as usize | PTEBit::User as usize,
        };
        (*root).entrys[1] = PTE {
            entry: 1 << 28
                | PTEBit::RWX as usize
                | PTEBit::Valid as usize
                | PTEBit::User as usize,
        };
        (*root).entrys[2] = PTE {
            entry: 0b10 << 28
                | PTEBit::RWX as usize
                | PTEBit::Valid as usize
                | PTEBit::User as usize,
        };
    }
    // let satp = 0x80000 << 44 | root as usize >> 12;
    // unsafe { asm!("csrw satp, {0}", in(reg)satp) }

    crate::switch::ecall_switch_to_context(unsafe { &mut crate::switch::CONTEXT_U });

    println!("[S][Info] 第二次");
    crate::switch::ecall_switch_to_context(unsafe { &mut crate::switch::CONTEXT_U });
    println!("[S][Info] 第三次");
    crate::switch::ecall_switch_to_context(unsafe { &mut crate::switch::CONTEXT_U });

    loop {}
}

// #[inline]
// fn sret() {
//     unsafe {
//         asm!("sret", options(noreturn));
//     }
// }

#[inline]
pub fn read_sstatus() -> usize {
    let sstatus: usize;
    unsafe {
        asm!("csrr {0}, sstatus", out(reg)sstatus);
    };
    sstatus
}

// #[inline]
// fn set_sstatus(sstatus: usize) {
//     unsafe {
//         asm!("csrw sstatus, {0}", in(reg)sstatus);
//     }
// }

// #[allow(unused)]
// enum SPP {
//     User,
//     Supervisor,
// }

// #[inline]
// fn set_spp(spp: SPP) {
//     // let mpp = mpp as usize & 0x3;
//     set_sstatus(read_sstatus() | ((spp as usize) << 8));
// }

// #[inline]
// fn set_sepc(sepc: usize) {
//     unsafe {
//         asm!(
//             "csrw sepc, {0}", in(reg)sepc
//         );
//     }
// }
