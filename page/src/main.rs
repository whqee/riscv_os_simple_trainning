#![no_main]
#![no_std]
#![feature(default_alloc_error_handler)]
// #![feature(alloc_error_handler)]

extern crate alloc;
use alloc::{string::String, boxed::Box};

mod cpu;
mod entry;
// mod mmu;
mod page;
mod pmp;
// mod smode;
mod switch;
mod trap;
mod uart;
// mod umode;

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

    // Physical Memory Page Allocator init.
    // page::init();
    page::初始化();

    {
        let p = Box::new("TTTT");
        let str = String::from("Test String");

        println!("{}, {}", str, p);
    }


    let f = 0.0;
    println!("f: {}", f);
    // let out = f.sin();

    // drop(str);

    // The demo is done. End with panic.
    panic!("Unreachable here.");
}
