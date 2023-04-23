// 0 ~ 0x8000_0000 = [0 ~ 2 GB)
// 0x8000_0000 ~ 0x8800_0000 = 128M = 64 x 2M = 64 x 512 x 4K
// Third Gigabit: {
//      0~63 2M (512 x 4K)
// }

// 0x8020_0004 = 0b1000 0000 0010 0000 _ 0000 0000 0000 0100
//             = sv39 0b 000 0000 1000 0000 0010 0000 _ 0000 0000 0000 0100
//             = sv39 0b 000000010 000000001 000000000 _ 0000 0000 0100

// VPN[2] = 000000010
// VPN[1] = 000000001
// VPN[0] = 000000000
// 4K Page offset = 0000 0000 0100

// set satp.PPN = (0x8020_0000 << 12)
//              = (0b 0000 0000 _ 0000 0000 0000 0000 _ 1000 0000 0010 0000 _ 0000 0000 0000 0000 << 12)
//              = (0b 0000 0000 _ 0000 0000 0000 0000 _ 1000 0000 0010 0000 _ 0000)
//              = (0b 0000 0000 0000 0000 0000 0000 10 _ 00 0000 001 _ 0 0000 0000)
// So, Sv39, satp = [1000 0*16 satp.PPN]

use core::{arch::asm, ops::BitAnd};

#[allow(unused)]
pub enum PTEBit {
    None,
    Valid = 1 << 0,
    R = 1 << 1,
    RW = 1 << 1 | 1 << 2,
    X = 1 << 3,
    RX = 1 << 1 | 1 << 3,
    RWX = 1 << 1 | 1 << 2 | 1 << 3,
    RWXU = 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
    User = 1 << 4,
    Global = 1 << 5,
    Accessed = 1 << 6,
    Dirty = 1 << 7,
}

#[cfg(target_arch = "riscv64")]
#[derive(Clone)]
/// Page Table Entry
pub struct PTE {
    pub entry: usize,
}

pub struct PageTable {
    pub entrys: [PTE; 512],
}

impl PTEBit {
    pub fn val(self) -> usize {
        self as usize
    }
}

impl PTE {
    pub fn is_valid(&self) -> bool {
        self.entry.bitand(!1usize) != 0
    }
    pub fn is_invalid(&self) -> bool {
        self.entry.bitand(!1usize) == 0
    }
    pub fn ppn(&self) -> usize {
        self.entry >> 10 & 0xFFF_FFFF_FFFF
    }
}

// impl PageTable {
//     pub fn clear(&mut self) {
//         (0..512).map(|x| self.entrys[x].entry = 0);
//     }
// }

/// root:   root of PageTable
/// vaddr:  virtual address
/// paddr:  physical address
/// bits:   bits 0-7 of PageTableEntry
/// level:  Page Table Entry level
///
/// detail: see riscv-privileged.pdf: Virtual-Memory System
fn __sv39_map(
    root: PageTable,
    vaddr: usize,
    paddr: usize,
    permission: PTEBit,
    level: usize,
    user: bool,
) {
    let ppn = [paddr >> 12, paddr >> (12 + 9), paddr >> (12 + 9 + 9)];
    let vpn = [vaddr >> 12, vaddr >> (12 + 9), vaddr >> (12 + 9 + 9)];
    for i in (0..=2).rev() {}
}

pub fn map() {}

/// A test to map 0x8000_0000 - 0x8800_0000 (128M) to itself for Supervisor Mode
#[inline]
#[allow(unused)]
pub fn mmu_mapping_test_for_s() {
    let root = 0x8020_0000 as *mut PageTable;
    
    // 0 - 0x8000_0000 2GB space and 0x8.. self-mapping
    // 0x4000_0000 = 0b .. 1 000000000(9bit) 000000000(9bit) _ 0000 0000 0000(12bit 4K) = 1 << 30
    // 0x8000_0000 = 1 << 31 = 0b10 << 30
    unsafe {
        // Clear the Page
        (0..512).map(|x| (*root).entrys[x].entry = 0);
        (*root).entrys[0] = PTE {
            entry: 0 | PTEBit::RWX.val() | PTEBit::Valid.val(),
        };
        (*root).entrys[1] = PTE {
            entry: 1 << 28 | PTEBit::RWX.val() | PTEBit::Valid.val(),
        };
        // 1 << 31 map the whole Gigabit
        // (*root).entrys[2] = PTE {
        //     entry: 0b10 << 28
        //         | PTEBit::RWX.val()
        //         | PTEBit::Valid.val(),
        // };

        // map per 4K, ~0x8800_0000 128M, just a test
        {
            // the third Gigabit (level 2)
            // next level(1) PageTable phy address: 0x8020_1000
            //                      = 0b1000 0000 0010 0000 _ 0001 0000 0000 0000
            //
            // PTE: 0b 0.. 10 0000 0000 1000 0000 0100 0000 0000
            // PTE: 0b 0.. 0x2008_0400
            (*root).entrys[2] = PTE {
                entry: 0x2008_0400 | PTEBit::Valid.val() | PTEBit::Global.val(),
            };

            // 64 x 4K for 2M PTE, 1 x 4K for G PTE, 1 x 4K for root PTE
            // Anyway, the third G Page Table = [0x8020_1000, 0x8020_2000)
            // let the 2M PTE entry starts with 0x8020_2000, Page Tables end with +256K=0x40000
            // Level 1 PTE
            for i in 0..64 {
                // next level(0) PTE physical address: 0x8020_2000 + offset(i*8), PTE = 0x2008_0800
                // 三级PTE， PPN为映射的[0x8000_0000,0x8800_0000)的0x800=2048个4K页基址
                // 2M = 0x20_0000
                let next_2m_pte = 0x8020_2000 + i * 0x1000;
                let leaf_pte_ppn_start = 0x8000_0000 + 0x20_0000 * i;

                for j in 0..512 {
                    (*(next_2m_pte as *mut PageTable)).entrys[j] = PTE {
                        entry: (leaf_pte_ppn_start + j * 0x1000) >> 2
                            | PTEBit::RWX.val()
                            | PTEBit::Valid.val()
                            | PTEBit::Global.val(),
                    }
                }

                (*(0x8020_1000 as *mut PageTable)).entrys[i] = PTE {
                    entry: next_2m_pte >> 2 | PTEBit::Valid.val() | PTEBit::Global.val(),
                };
            }
        }
    }
    let satp = 0x80000 << 44 | root as usize >> 12;
    unsafe { asm!("csrw satp, {0}", in(reg)satp) }
}
