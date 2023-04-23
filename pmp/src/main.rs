#![no_main]
#![no_std]

mod cpu;
mod entry;
mod pmp;
mod smode;
mod uart;

#[no_mangle]
fn main() -> ! {
    entry::zero_bss();
    uart::Uart::new(0x1000_0000).init();
    println!("Hello, world!");

    // switch to smode
    cpu::MPP::set(cpu::MPP::Supervisor);
    cpu::set_mepc(smode::smain as usize);
    pmp::init();

    println!("[Debug] mret to s mode ...");
    mret!();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {{
        use core::fmt::Write;
        let _ = (0x1000_0000 as uart::Uart).write_fmt(format_args!($fmt $(, $($arg)+)?));
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
