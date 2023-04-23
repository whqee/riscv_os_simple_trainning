#![no_main]
#![no_std]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(asm_sym)]

// mod entry;
// mod driver;
// mod console;
// mod page;

// use exercises::*;
// core::arch::global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn main() -> ! {
    console::init();
    entry::zero_bss();
    // run_a_task();
    // run_three_task();
    println!("Hello Rust!");
    loop {
        while let Some(c_ascii) = console::read_byte() {
            println!("read a ascii: {}", c_ascii);
        }
    }
}

// S mode main()
pub fn smain() -> ! {
    
    todo!()
}


#[no_mangle]
#[link_section = ".text.init"]
pub fn _start() -> ! {
    unsafe {
        asm!(
            "   csrr t0, mhartid
                beqz t0, 2f
            1:  wfi
                j 1b
            2:  csrw mie, zero

                la sp, {stack}
                li t0, 0x20000
                add sp, sp, t0
            ",
            stack = sym STACK
        );
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
