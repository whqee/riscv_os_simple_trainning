use crate::println;
use core::{arch::asm, panic::PanicInfo};

#[no_mangle]
#[link_section = ".text.init"]
pub fn _start() -> ! {
    // extern "Rust" {
    //     fn main() -> !;
    // }
    unsafe {
        // hart 0 continue, the others do nothing
        asm!(
            "
            vmul.vx v2,v1,t0,v0.t
                    la sp, m_stack_top
                "
        );
    }
    crate::main()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[?][Panic] {}", info);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

pub fn zero_bss()
where
    dyn Fn() -> (): FnOnce(),
{
    let bss_start: usize;
    let bss_end: usize;
    unsafe { asm!("la {0}, _bss_start", "la {1}, _bss_end", out(reg)bss_start, out(reg)bss_end) }

    println!(
        "[Debug BSS] BSS_START = 0x{:X}, BSS_END = 0x{:X}, Size = 0x{:X}",
        bss_start,
        bss_end,
        bss_end - bss_start
    );

    if bss_start > bss_end {
        panic!(".bss wrong, please check linker-file!");
    }

    for addr in bss_start..bss_end {
        unsafe { (addr as *mut u8).write_volatile(0) }
    }
}
