use core::{arch::asm, panic::PanicInfo};

use crate::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[Panic]: {}", info);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[no_mangle]
#[link_section = ".text.init"]
pub fn _start() -> ! {
    // extern "Rust" {
    //     fn main() -> !;
    // }

    #[link_section = ".bss.stack"]
    static mut STACK: [u8; 0x20000] = [0; 0x20000];
    // #[cfg(target_arch = "riscv64")]
    // disable INT and set stack pointer
    // asm!("csrw mie, zero", "la sp, kernel_stack_top");

    unsafe {
        asm!(
            "   csrr t0, mhartid
                beqz t0, 2f
            1:  wfi
                j 1b
            2:  csrw mie, zero
                .option push
                .option norelax
                    la  gp, _global_pointer
                .option pop
            
                la sp, {stack}
                li t0, 0x20000
                add sp, sp, t0
            ",
            stack = sym STACK
        );

        #[cfg(target_arch = "aarch64")]
        asm!("wfe");

        crate::main()
    }
}

pub fn zero_bss()
where
    dyn Fn() -> (): FnOnce(),
{
    extern "C" {
        static mut sbss: usize;
        static mut ebss: usize;
    }
    unsafe {
        println!("[Debug]: sbss={}, ebss={}", sbss, ebss);
        if sbss > ebss {
            panic!(".bss wrong, please check linker-file!");
        }
        for addr in sbss..ebss {
            (addr as *mut u8).write_volatile(0);
        }
    }
}
