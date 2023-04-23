#![no_main]
#![no_std]
// #![feature(default_alloc_error_handler)]
// #![feature(alloc_error_handler)]

// extern crate alloc;
// use alloc::{string::String, boxed::Box};

use core::{arch::asm, mem::size_of};

use crate::{
    mmu::{map, PageTable, Sv39Satp},
    page::{zalloc, PAGE_SIZE},
    switch::{CONTEXT_APP_HELLO, CONTEXT_U},
};

mod cpu;
mod entry;
mod mmu;
mod page;
mod pmp;
// mod smode;
mod switch;
mod trap;
mod uart;
mod umode;
mod sbi;
mod console;

#[no_mangle]
fn main() -> ! {
    entry::zero_bss();
    println!("[S][Info] Hello, Rust!");
    // uart::Uart::new(0x1000_0000).init();
    println!("[S][Info] Hello, Rust!");

    // set s mode trap with ecall handler
    trap::init();

    // set sscratch for cpu0
    cpu::set_sscratch(unsafe { [0usize; 0x2000].as_ptr().add(0x2000) as usize });
    cpu::enable_s_interrupt();

    // Physical Memory Protection (Required by S/U Mode), unneeded in s-mode
    // pmp::init();

    // Physical Memory Page Allocator init.
    page::init();

    // {
    //     let p = Box::new("TTTT");
    //     let str = String::from("Test String");
    //     println!("{}, {}", str, p);
    // }
    unsafe {
        // let _root = PageTable::new();
        // println!("_root ptr = 0x{:X}", _root.as_ptr() as usize);
        // let _root = unsafe { &mut *_root };
        // page::dealloc(_root as *mut u8);

        // let _a = page::alloc(0x7f61);
        // if !_a.is_null() {
        //     page::dealloc(_a);
        // }
    }

 
    // unmap!(&mut root,0,0);

    // drop(str);

    // The demo is done. End with panic.
    panic!("Unreachable here.");
}
