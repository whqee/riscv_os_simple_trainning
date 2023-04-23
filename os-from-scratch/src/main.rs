#![no_main]
#![no_std]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(asm_sym)]

mod entry;
mod driver;
mod console;
// mod page;

core::arch::global_asm!(include_str!("link_app.S"));

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
            if c_ascii == '1' as u8 {
                extern "Rust" {
                    fn _start();
                }
                unsafe {
                    _start();
                }
            }
            if c_ascii == 'b' as u8 {
                break;
            }
        }
    }
}

// S mode main()
pub fn smain() -> ! {
    
    todo!()
}

