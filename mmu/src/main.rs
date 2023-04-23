#![no_main]
#![no_std]

use core::arch::asm;

mod cpu;
mod entry;
mod pmp;
mod switch;
mod trap;
mod uart;
mod smode;
mod mmu;
mod umode;

#[no_mangle]
fn main() -> ! {
    entry::zero_bss();
    uart::Uart::new(0x1000_0000).init();
    println!("[M][Info] Hello, Rust!");

    // set m mode trap with ecall handler
    trap::init();

    // set mscratch for cpu0
    cpu::set_mscratch(unsafe { [0usize; 0x2000].as_ptr().add(0x2000) as usize });
    cpu::enable_m_interrupt();

    // Physical Memory Protection (Required by S/U Mode)
    pmp::init();

    // Enable MMU (simple test)
    mmu::mmu_mapping_test_for_s();

    unsafe {
        switch::CONTEXT_S.frame.sp = [0usize; 0x2000].as_ptr().add(0x2000) as usize;
        switch::CONTEXT_S.frame.mepc = smode::smain as usize;
        switch::CONTEXT_S.mstatus = cpu::MStatus {
            raw: cpu::MStatus::read().raw & !(3usize << 11)
                | ((cpu::MPP::Supervisor as usize) << 11),
        };
        switch::CONTEXT_S.satp = 0x8020_0000;

        switch::CONTEXT_U.frame.sp = [0usize; 0x2000].as_ptr().add(0x2000) as usize;
        switch::CONTEXT_U.frame.mepc = umode::umain as usize;
        switch::CONTEXT_U.mstatus = cpu::MStatus {
            raw: cpu::MStatus::read().raw & !(3usize << 11) | ((cpu::MPP::User as usize) << 11),
        };
    }

    println!("[M][Info] ecall Test");
    switch::ecall_switch_to_context(unsafe { &mut switch::CONTEXT_S });
    switch::ecall_switch_to_context(unsafe { &mut switch::CONTEXT_S });

    // The demo is done. End with panic.
    panic!("Unreachable here.");
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {{
        use core::fmt::Write;
        let _ = crate::uart::Uart::new(0x1000_0000).write_fmt(format_args!($fmt $(, $($arg)+)?));
    }}
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {{
        use core::fmt::Write;
        use crate::uart::Uart;
        let _ = Uart::new(0x1000_0000).write_fmt(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }}
}
