#![no_main]
#![no_std]

use core::arch::asm;

mod console;
mod cpu;
mod entry;
mod pmp;
mod smode;
mod switch;
mod trap;
mod uart;
mod umode;

#[no_mangle]
fn main() -> ! {
    entry::zero_bss();
    console::init();
    // uart::Uart::new(0x1000_0000).init();
    println!("[M][Info] Hello, world!");

    // set m mode trap with ecall handler
    trap::init();

    // set mscratch for cpu0
    cpu::set_mscratch(unsafe { [0usize; 0x2000].as_ptr().add(0x2000) as usize });
    cpu::enable_m_interrupt();

    // Physical Memory Protection (Required by S/U Mode)
    pmp::init();

    // let cx_s = switch::new_simple_context(
    //     unsafe { [0usize; 0x2000].as_ptr().add(0x2000) as usize },
    //     smode::smain as usize,
    //     cpu::MPP::Supervisor,
    // );

    unsafe {
        // switch::CONTEXT_S.clone_from(&cx_s);
        // switch::CONTEXT_U.clone_from(&cx_u);

        switch::CONTEXT_S.frame.sp = [0usize; 0x2000].as_ptr().add(0x2000) as usize;
        switch::CONTEXT_S.frame.mepc = smode::smain as usize;
        switch::CONTEXT_S.mstatus = cpu::MStatus {
            raw: cpu::MStatus::read().raw & !(3usize << 11)
                | ((cpu::MPP::Supervisor as usize) << 11),
        };

        switch::CONTEXT_U.frame.sp = [0usize; 0x2000].as_ptr().add(0x2000) as usize;
        switch::CONTEXT_U.frame.mepc = umode::umain as usize;
        switch::CONTEXT_U.mstatus = cpu::MStatus {
            raw: cpu::MStatus::read().raw & !(3usize << 11) | ((cpu::MPP::User as usize) << 11),
        };
    }
    // drop(cx_s);

    println!("[M][Info] ecall Test");
    // switch::ecall_switch_to_context(unsafe { &mut switch::CONTEXT_S });
    unsafe {
        let a7 = 1usize;
        asm!("ecall", in("a0")(&switch::CONTEXT_S), in("a7")a7)
        // asm!("ecall")
    }

    println!("str1 0123_4567");

    println!("str2 9999_9999");
    println!("str1 0123_4567");

    switch::ecall_switch_to_context(unsafe { &mut switch::CONTEXT_S });

    println!("str2 9999_9999");
    println!("str1 01234567");

    // switch::ecall_switch_to_context(unsafe { &mut switch::CONTEXT_S });

    // 'pc' will switch back here by ecall handler
    // todo!();

    // unsafe {
    //     let v = 0x0 as *mut u64;
    //     v.write_volatile(0);
    // }

    // The demo is done. End with panic.
    panic!("Unreachable here.");
}

// #[macro_export]
// macro_rules! print {
//     ($fmt: literal $(, $($arg: tt)+)?) => {{
//         use core::fmt::Write;
//         let _ = crate::uart::Uart::new(0x1000_0000).write_fmt(format_args!($fmt $(, $($arg)+)?));
//     }}
// }

// #[macro_export]
// macro_rules! println {
//     ($fmt: literal $(, $($arg: tt)+)?) => {{
//         use core::fmt::Write;
//         use crate::uart::Uart;
//         let _ = Uart::new(0x1000_0000).write_fmt(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
//     }}
// }
