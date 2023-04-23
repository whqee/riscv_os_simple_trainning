use crate::{cpu, println};

core::arch::global_asm!(include_str!("trap.S"));

pub fn init() {
    // Rust-ABI is not stable yet, but it's Ok to the function without parameters.
    extern "Rust" {
        fn __m_trap_entry();
    }
    cpu::set_mtvec_direct_mode(__m_trap_entry as usize);
}

#[no_mangle]
// Called by trap.S: __m_trap_entry
pub fn m_trap_handler() {
    println!("mcause: 0x{:x}", cpu::Mcause::mcause());
    println!("mip: 0x{:x}", cpu::mip());
    cpu::set_mip(0);
    println!("mip: 0x{:x}", cpu::mip());

    cpu::set_timer(cpu::get_time() + 0x6670000);
    println!("mip: 0x{:x}", cpu::mip());




    static mut COUNT: i32 = 0;
    unsafe {
        COUNT = COUNT + 1;
        if COUNT > 2 {
            todo!()
        }
    }
}
