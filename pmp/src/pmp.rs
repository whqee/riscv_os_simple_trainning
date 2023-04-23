// Referrence: riscv-privileged.pdf :  Physical Memory Protection
// CSRs: (usize)
// pmpcfg0~15: CSR 0x3A0~0x3AF   (config slot: 0~63 corresponding to pmpaddr:0~63)
// pmpaddr[0~63]: CSR 0x3B0~0x3EF

// For example, riscv64:
//
// 1. pmpcfg0 64bits: [pmp7cfg, pmp6cfg, pmp5cfg, pmp4cfg, pmp3cfg, pmp2cfg, pmp1cfg, pmp0cfg], corresponding
//  to pmpaddr[0..=7]
//
//  pmpXcfg 8bits: [L 00 A[2] X W R], L=Lock?, A='Address Matching', X=fetch, W=Store/AMO, R=Load
//
// 'Address Matching': 0 -- OFF: Null region (disabled)
//                     1 -- TOR: Top of range
//                     2 -- NA4: Naturally aligned four-byte region
//                     3 -- NAPOT: Naturally aligned power-of-two(2的平方) region, >= 8 Bytes
//
// 2. pmpaddr(i) 64 bit: [ 0[10] address[55:2] ]  '55:2': 56 bit address, 0~55 cast away 2 bits, (X >> 2)
//
// the range of pmpaddr(i): if (i is 0)  ==> [0, (pmpaddr0)],
//                                  else         ==> [(pmpaddr(i-1)), (pmpaddr(i))]
//
//
// 3. I want to set [0x8000_0000..0x8020_0000)'s permission to be 'RWX', then i can set :
//
//      pmp(i-1)cfg = [_], and pmpaddr(i) = (0x8000_0000 >> 2)
//      pmp(i)cfg = [_ 00 01 1 1 1], and pmpaddr(i) = (0x8020_0000 >> 2)
//
//

// set PMP, 0~0x8000_0000 noLock RW, 0x8000_0000~0x8800_0000 128MB noLock RWX
pub fn init()
where
    dyn Fn(): FnOnce() -> (),
{
    // pmpaddr0 = (0x8000_0000 >> 2)
    let val = 0x8000_0000usize >> 2;
    unsafe { core::arch::asm!("csrw pmpaddr0, {0}", in(reg)val) }

    // pmpaddr1 = (0x8800_0000 >> 2)   0x0800_0000 = 128MB
    let val = 0x8800_0000usize >> 2;
    unsafe { core::arch::asm!("csrw pmpaddr1, {0}", in(reg)val) }

    // pmp1cfg = [0 00 01 1 1 1], pmp0cfg = [0 00 01 0 1 1], others default to zero
    // so, pmpcfg0 = [0.. 0000_1111 0000_1011] = 0xF0B
    let val = 0xF0Busize;
    unsafe { core::arch::asm!("csrw pmpcfg0, {0}", in(reg)val) }
}

// pub mod pmpcfg0 {
//     pub fn () {

//     }
// }
