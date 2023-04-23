use core::arch::asm;

use crate::println;

// static S_STACK: [usize; 0x2000] = [0usize; 0x2000];

#[no_mangle]
pub fn smain() -> ! {
    println!("[S][Info] Hello !");
    println!("[S][Info] hello ! sstatus={:x}", read_sstatus() >> 8 & 0x1);
    // set_spp(SPP::User);
    // set_sepc(umode::umain as usize);
    // sret();

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
