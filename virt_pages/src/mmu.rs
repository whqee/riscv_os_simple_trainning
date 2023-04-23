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

use core::ops::{BitAnd, IndexMut};

use crate::{
    page::{dealloc, zalloc},
    println,
};

#[allow(unused)]
#[derive(PartialEq, Eq)]
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

#[allow(unused)]
#[derive(PartialEq, Eq)]
pub enum Permission {
    None,
    R = 1 << 1,
    RW = 1 << 1 | 1 << 2,
    X = 1 << 3,
    RX = 1 << 1 | 1 << 3,
    RWX = 1 << 1 | 1 << 2 | 1 << 3,
}

#[cfg(target_arch = "riscv64")]
#[derive(Clone,Debug)]
#[repr(C)]
/// Page Table Entry
pub struct PTE {
    pub entry: usize,
}

#[derive(Clone,Debug)]
#[repr(C)]
pub struct PageTable {
    pub entrys: [PTE; 512],
}

impl PTEBit {
    #[inline]
    pub fn val(self) -> usize {
        self as usize
    }
}
impl Permission {
    #[inline]
    pub fn val(self) -> usize {
        self as usize
    }
}

impl PTE {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.entry.bitand(!1usize) != 0
    }

    #[inline]
    pub fn is_invalid(&self) -> bool {
        self.entry.bitand(!1usize) == 0
    }

    #[inline]
    pub fn ppn(&self) -> usize {
        // self.entry >> 10 & 0xFFF_FFFF_FFFF
        (self.entry & (0xFFF_FFFF_FFFF << 10)) >> 10
    }

    #[inline]
    /// unsafe. this func do not check if PPN is valid !!
    pub unsafe fn ppn_as_pagetable_mut(&self) -> &mut PageTable {
        &mut *(((self.entry & (0xFFF_FFFF_FFFF << 10)) << 2) as *mut PageTable)
    }

    #[inline]
    pub fn is_leaf(&self) -> bool {
        let bits = self.entry & Permission::RWX.val();
        if bits == 1 << 2 {
            panic!("Invalid Permission bits ('W') at PTE");
        }
        bits != Permission::None.val()
    }

    #[inline]
    pub fn is_not_leaf(&self) -> bool {
        !self.is_leaf()
    }
}

impl PageTable {
    // /// maybe not a good use without Box supported
    // #[inline]
    // pub unsafe fn new_as_mut_ptr() -> *mut PageTable {
    //     zalloc(1) as *mut PageTable
    // }

    // #[inline]
    // pub fn new() -> Self {
    //     // let a = unsafe {
    //     //     *(zalloc(1) as *mut [PTE;512])
    //     // };

    //     let mut a = unsafe{*(zalloc(1) as *mut [usize;512])};
    //     for i in a {

    //     }
    //     let b: [PTEE;512] = unsafe {*(a.as_mut_ptr() as *mut [PTEE;512])};
    //     todo!()
    // }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut PageTable {
        self as *mut PageTable
    }

    #[inline]
    pub fn as_ptr(&self) -> *const PageTable {
        self as *const PageTable
    }

    #[inline]
    pub fn pte_mut(&mut self, index: usize) -> &mut PTE {
        self.entrys.index_mut(index)
    }

    // #[allow(unused)]
    // #[inline]
    // pub fn pte(&self, index: usize) -> &PTE {
    //     self.entrys.index(index)
    // }

    #[inline]
    pub fn to_a_branch_pte(&mut self) -> PTE {
        PTE {
            entry: (self.as_mut_ptr() as usize & !0xFFF) >> 2 | PTEBit::Valid.val(),
        }
    }
}

// test
// impl Drop for PageTable {
//     fn drop(&mut self) {
//         unsafe { dealloc(self.as_mut_ptr() as *mut u8) };
//         println!("drop a PageTable")
//     }
// }

/// root:   root of PageTable
/// vaddr:  virtual address
/// paddr:  physical address
/// bits:   bits 0-7 of PageTableEntry
/// level:  Page Table Entry level, 2->1GB, 1->2MB, 0->4K
///
/// detais see riscv-privileged.pdf: Virtual-Memory System
fn __sv39_map(
    root: &mut PageTable,
    vaddr: usize,
    paddr: usize,
    permission: Permission,
    level: usize,
    user: bool,
) {
    assert!(level <= 2);
    assert!(permission != Permission::None);
    let ppn = paddr & !0xFFF >> 2;

    let vpn = [
        (vaddr >> 12) & 0b1_1111_1111,
        (vaddr >> (12 + 9)) & 0b1_1111_1111,
        vaddr >> (12 + 9 + 9),
    ];

    // println!("vaddr 0x{:X}, {}, vpn {:?}", vaddr, vaddr >> 12, vpn);
    let mut leaf = root.pte_mut(vpn[2]);

    for i in (level..2).rev() {
        // get next pte in the next table, until target leaf pte
        if leaf.is_invalid() {
            // no next. alloc a Page to new one Pagetable
            let new_pagetable_ptr = unsafe { zalloc(1) } as *mut PageTable;

            unsafe {
                // entry.ppn.as_page_addr --pointer_to--> this new PageTable
                leaf.entry = (*new_pagetable_ptr).to_a_branch_pte().entry;
                leaf = (*new_pagetable_ptr).pte_mut(vpn[i]);
            }
        } else if leaf.is_leaf() {
            panic!("this PTE is leaf, in others using")
        } else {
            // else: not leaf PTE (is branch), get next level PTE
            leaf = unsafe { leaf.ppn_as_pagetable_mut() }.pte_mut(vpn[i])
        }
    }

    // check if the target leaf PTE is valid ? in others using ?
    if leaf.is_valid() {
        if leaf.is_not_leaf() {
            // if we cover it, the next table Page(s) will be lost and out of manage,
            // that will cause memory leak
            panic!("this PTE is branch, in others using")
        } else if leaf.ppn() != ppn {
            // comment bellow panic!() to allow remapping a page directly !!!
            panic!("this PTE is leaf, in others using. leaf.ppn() = 0x{:x}, ppn = 0x{:x}", leaf.ppn(), ppn)
        } else {
            // already mapped, return directly
            println!("already mapped, return directly");
            return;
        }
    }

    // set leaf PTE and done
    leaf.entry = ((paddr & !0xFFF) >> 2)
        | permission.val()
        | ((user as usize) << 4)
        | PTEBit::Valid.val()
        | PTEBit::Accessed.val()
        | PTEBit::Dirty.val();
}

fn __sv39_unmap(root: &mut PageTable, vaddr: usize, level: usize) {
    assert!(level <= 2);
    let vpn = [
        (vaddr >> 12) & 0b1_1111_1111,
        (vaddr >> (12 + 9)) & 0b1_1111_1111,
        vaddr >> (12 + 9 + 9),
    ];

    let mut leaf = root.pte_mut(vpn[2]);

    for i in (level..2).rev() {
        if leaf.is_invalid() || leaf.is_leaf() {
            // it's unmaped now
            return;
        }
        // next level PTE, maybe leaf PTE or not
        leaf = unsafe { leaf.ppn_as_pagetable_mut() }.pte_mut(vpn[i]);
    }

    if leaf.is_invalid() {
        // It's already unmapped. Should we panic!() here ? // No need to panic!()
        return;
    }

    if leaf.is_not_leaf() {
        // should we support this ? ..?
        // __sv39_unmap_table(&mut PageTable::from(*leaf));
        todo!()
    }

    leaf.entry = 0;
}

fn __sv39_unmap_table(table: &mut PageTable) {
    for i in 0..512 {
        if table.entrys[i].is_valid() && table.entrys[i].is_not_leaf() {
            __sv39_unmap_table(unsafe { table.entrys[i].ppn_as_pagetable_mut() })
        }
    }
    unsafe { dealloc((table as *mut PageTable) as *mut u8) }
}

/// root:   root of PageTable
///
/// vaddr:  virtual address
///
/// paddr:  physical address
///
/// level:  Page Table Entry level, 2->1GB, 1->2MB, 0->4K
///
/// detais see riscv-privileged.pdf: Virtual-Memory System
pub fn map(
    root: &mut PageTable,
    vaddr: usize,
    paddr: usize,
    permission: Permission,
    level: usize,
    user: bool,
) {
    // ... Reserved ...
    // mapping
    // unsafe {
    __sv39_map(root, vaddr, paddr, permission, level, user);
    // }
}

pub fn unmap(root: &mut PageTable, vaddr: usize, level: usize) {
    __sv39_unmap(root, vaddr, level)
}

pub fn unmap_table(table: &mut PageTable) {
    __sv39_unmap_table(table)
}

// trait MMU<T> {
//     fn unmap(t: T);
// }

#[macro_export]
/// unmap(root: &mut PageTable, vaddr: usize, level: usize)
/// or
/// unmap(table: &mut PageTable)
macro_rules! unmap {
    ($root:expr, $vaddr:expr, $level:expr) => {
        crate::mmu::unmap($root, $vaddr, $level);
    };
    ($table:expr) => {
        crate::mmu::unmap_table($table);
    };
}

pub struct Sv39Satp(usize);

impl Sv39Satp {
    pub fn usize(&self) -> usize {
        self.0
    }
}

impl From<*mut PageTable> for Sv39Satp {
    fn from(t: *mut PageTable) -> Self {
        Self(1 << 63 | t as usize >> 12)
    }
}

/// A test to map 0x8000_0000 - 0x8800_0000 (128M) to itself for Supervisor Mode
#[inline]
#[allow(unused)]
pub fn mmu_mapping_test_for_s() -> usize {
    // let root = 0x8020_0000 as *mut PageTable;
    let root = unsafe { zalloc(1) } as *mut PageTable;

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
    unsafe { core::arch::asm!("csrw satp, {0}", in(reg)satp) };
    satp
}
