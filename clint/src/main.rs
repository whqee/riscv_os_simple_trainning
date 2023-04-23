#![no_main]
#![no_std]

mod clint;
mod cpu;
#[allow(unused)]
// #![feature(naked_functions, asm_sym, asm_const)]
mod entry;
mod trap;
mod uart;

#[no_mangle]
fn main() -> ! {
    entry::zero_bss();
    uart::Uart::new(0x1000_0000).init();
    println!("Hello, world!");

    // set trap handler
    trap::init();

    // set mscratch
    static mut M_TRAP_CPU0_STACK: [usize; 32768] = [0usize; 0x8000];
    let m_trap_cpu0_sp: usize;
    unsafe {
        m_trap_cpu0_sp = M_TRAP_CPU0_STACK.as_ptr().offset(0x8000) as usize;
    }
    if cpu::mhartid() == 0 {
        cpu::set_mscratch(m_trap_cpu0_sp);
    }

    // enable Timer INT
    cpu::enable_m_timer_interrupt();

    // enable INT
    cpu::enable_m_interrupt();

    // set timer time_val
    cpu::set_timer(cpu::get_time() + 0x067_0000);

    // unsafe {
    //     let v = 0x0 as *mut u64;
    //     v.write_volatile(0);
    // }

    loop {
        if let Some(c_ascii) = uart::Uart::new(0x1000_0000).get_byte() {
            println!("read a ascii: {}", c_ascii);
        }
    }
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
