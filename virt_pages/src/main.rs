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

    unsafe {
        CONTEXT_U.stack = zalloc(1);
        CONTEXT_U.root = zalloc(1) as *mut PageTable;
        CONTEXT_U.mem = zalloc(1);

        // map device space to it, including Uart0
        map(
            CONTEXT_U.root.as_mut().unwrap(),
            0x10000000,
            0x10000000 as usize,
            mmu::Permission::RW,
            0,
            true,
        );

        extern "C" {
            fn __c_char_write_test();
        }
        // 搞段.text过来
        CONTEXT_U
            .mem
            .copy_from(__c_char_write_test as usize as *mut u8, PAGE_SIZE);

        CONTEXT_U.program_start = 0xf8000_0000 as usize + PAGE_SIZE;

        // map .text
        map(
            CONTEXT_U.root.as_mut().unwrap(),
            CONTEXT_U.program_start,
            CONTEXT_U.mem as usize,
            mmu::Permission::RWX,
            0,
            true,
        );
        // println!("{:X?}", CONTEXT_U.root.as_ref().unwrap().entrys);
        // println!("{:X?}", CONTEXT_U.root.as_ref().unwrap().entrys[2].ppn_as_pagetable_mut().as_ptr() as usize);
        // println!("{:X?}", CONTEXT_U.root.as_ref().unwrap().entrys[2].ppn_as_pagetable_mut());
        // println!("{:X?}", CONTEXT_U.root.as_ref().unwrap().entrys[2].ppn_as_pagetable_mut().entrys[0].ppn_as_pagetable_mut().as_ptr() as usize);
        // println!("{:X?}", CONTEXT_U.root.as_ref().unwrap().entrys[2].ppn_as_pagetable_mut().entrys[0].ppn_as_pagetable_mut().entrys[1].ppn_as_pagetable_mut().as_ptr() as usize);
        // panic!();

        // map stack space, 预设frame.sp
        let stack_end = 0x8020_0000;
        map(
            CONTEXT_U.root.as_mut().unwrap(),
            stack_end,
            CONTEXT_U.stack as usize,
            mmu::Permission::RWX,
            0,
            true,
        );
        CONTEXT_U.frame.sp = stack_end + 1 * PAGE_SIZE - 1;

        // 预设User Mode
        CONTEXT_U.frame.mstatus = cpu::MStatus {
            raw: cpu::MStatus::read().raw & !(3usize << 11) | ((cpu::MPP::User as usize) << 11),
        };

        // 预设mepc
        CONTEXT_U.frame.mepc = CONTEXT_U.program_start;

        // 预设satp， 开启MMU Sv39模式
        CONTEXT_U.frame.satp = Sv39Satp::from(CONTEXT_U.root).usize();
    }
    println!("");
    switch::ecall_switch_to_context(unsafe { &mut CONTEXT_U });
    println!("e");
    switch::ecall_switch_to_context(unsafe { &mut CONTEXT_U });
    println!("e");
    // switch::ecall_switch_to_context(unsafe { &mut CONTEXT_U });

    // new and switch to U mode to run linux APP 'print_hello' by hand
    unsafe {
        // let root_table = (zalloc(1) as *mut PageTable).as_mut().unwrap();
        // println!("root_table ptr = 0x{:X}\n", root_table.as_ptr() as usize);

        CONTEXT_APP_HELLO.root = zalloc(1) as *mut PageTable;
        // preset size
        let stack_size = 1024 * PAGE_SIZE; // 1024*4K = 4M
        CONTEXT_APP_HELLO.stack = zalloc(stack_size / PAGE_SIZE);
        let main_stack_sp = 0x0000003ffffff000;

        for i in 0..stack_size / PAGE_SIZE {
            map(
                CONTEXT_APP_HELLO.root.as_mut().unwrap(),
                main_stack_sp - i * PAGE_SIZE,
                CONTEXT_APP_HELLO.stack as usize + i * PAGE_SIZE,
                mmu::Permission::RW,
                0,
                true,
            );
        }
        CONTEXT_APP_HELLO.frame.sp = main_stack_sp as usize;

        let app_0_start: usize;
        let app_0_end: usize;
        asm!(
            "la {0}, app_0_start
            la {1}, app_0_end",
            out(reg)app_0_start,
            out(reg)app_0_end
        );
        let app_size = app_0_end - app_0_start;

        // let app_size = 0xbfa7e;

        println!(
            "app0 start = 0x{:X} 101c8, end = 0x{:X}, size = 0x{:X}",
            app_0_start, app_0_end, app_size
        );

        // let pages = (0x80000 + PAGE_SIZE) / PAGE_SIZE;
        let pages = (app_size + PAGE_SIZE) / PAGE_SIZE;
        // CONTEXT_APP_HELLO.mem = zalloc(pages);
        CONTEXT_APP_HELLO.mem = zalloc(pages);
        // let mem = zalloc(pages);
        // load app
        // mem.copy_from(app_0_start as *mut u8, app_size);
        CONTEXT_APP_HELLO
            .mem
            .add(0x1c8)
            .copy_from(app_0_start as *mut u8, app_size);

        // for i in 0x224..(0x224 + 40) {
        //     print!("{:02x} ", CONTEXT_APP_HELLO.mem.add(i).read());
        // }
        // println!("");
        // print!(
        //     "{:02x} ",
        //     (CONTEXT_APP_HELLO.mem.add(0x224 + 0x1EFC) as *mut usize).read()
        // );
        // print!(
        //     "{:02x} ",
        //     (CONTEXT_APP_HELLO.mem.add(0x224 + 0x1EFC + 0x20) as *mut u32).read()
        // );

        // println!("eb 0x{:X?}", *(mem.add(0x39C) as *const u16));

        // let text_start = 0x10224_usize;
        // CONTEXT_APP_HELLO.program_start = 0x80600000;
        // map the vaddr to paddr for .text
        // for i in 0..((app_size + PAGE_SIZE) / PAGE_SIZE) {
        // for i in 0..200 {
        for i in 0..pages {
            map(
                CONTEXT_APP_HELLO.root.as_mut().unwrap(),
                // .text, checked by objdump
                0x10000 + i * PAGE_SIZE,
                CONTEXT_APP_HELLO.mem as usize + i * PAGE_SIZE,
                mmu::Permission::RWX,
                0,
                true,
            );
        }

        // let a0 = zalloc(0x10);
        // for i in 0..0x10 {
        //     map(
        //         CONTEXT_APP_HELLO.root.as_mut().unwrap(),
        //         0x0 + i * PAGE_SIZE,
        //         a0 as usize + i * PAGE_SIZE,
        //         mmu::Permission::RWX,
        //         0,
        //         true,
        //     );
        // }

        let a1 = zalloc(1);
        map(
            CONTEXT_APP_HELLO.root.as_mut().unwrap(),
            0x7a2e8,
            a1 as usize,
            mmu::Permission::RWX,
            0,
            true,
        );

        CONTEXT_APP_HELLO.frame.mstatus = cpu::MStatus {
            raw: cpu::MStatus::read().raw & !(3usize << 11) | ((cpu::MPP::User as usize) << 11),
        };

        CONTEXT_APP_HELLO.program_start = 0x10544;
        CONTEXT_APP_HELLO.frame.mepc = CONTEXT_APP_HELLO.program_start;

        CONTEXT_APP_HELLO.frame.satp = Sv39Satp::from(CONTEXT_APP_HELLO.root).usize();
    }

    switch::ecall_switch_to_context(unsafe { &mut CONTEXT_APP_HELLO });

    // unmap!(&mut root,0,0);

    // drop(str);

    // The demo is done. End with panic.
    panic!("Unreachable here.");
}
