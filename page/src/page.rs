use core::ptr::null_mut;

use crate::println;

const PAGE_SIZE: usize = 4096;
static mut ALLOC_START: usize = 0;
static mut ALLOC_END: usize = 0;
static mut ALLOC_PAGES_SUM: usize = 0;
static mut PAGES_ENTRY: *mut Page = null_mut();

/// 1) is this page taken? and
/// 2) is this the last page of a contiguous allocation? (when we free them)
#[repr(u8)]
enum PageBit {
    Free,
    Taken = 1 << 0,
    Last = 1 << 1,
}

impl PageBit {
    pub fn val(self) -> u8 {
        self as u8
    }
}

struct Page {
    flag: u8,
}

impl Page {
    fn is_taken(&self) -> bool {
        self.flag == PageBit::Taken.val()
    }

    fn is_free(&self) -> bool {
        self.flag == PageBit::Free.val()
    }

    fn clear(&mut self) {
        self.flag = PageBit::Free.val()
    }

    fn set_flag(&mut self, flag: PageBit) {
        self.flag |= flag.val()
    }

    fn clear_flag(&mut self, flag: PageBit) {
        self.flag &= !flag.val()
    }
}

#[inline]
pub fn 初始化() {
    init()
}

pub fn init()
where
    dyn Fn() -> (): FnOnce(),
{
    // 1 Byte to store PageBit. 1B declaims 1 x 4K. How much memory needed to store PageBits ?
    // The heap should be aligned by 4K.

    let heap_start: usize;
    let heap_size: usize;

    unsafe {
        core::arch::asm!("
            la {0}, _heap_start
            la {1}, _heap_size
            ", 
            out(reg)heap_start,
            out(reg)heap_size);
    }

    // free bytes and pages ? and free pages left for allocator ?
    // let heap_start be the Page start(entry), so,
    let mut free_pages = heap_size / 4096;
    let mut gap = ((heap_start + PAGE_SIZE) & !(PAGE_SIZE - 1)) - heap_start;

    while gap < free_pages {
        gap += PAGE_SIZE;
        free_pages -= 1;
    }

    let alloc_start = heap_start + gap;
    let pages_entry = heap_start as *mut Page;

    println!(
        "[M][Info Heap Init] heap_start = 0x{:x}, heap_size = 0x{:x}, free_pages = 0x{:x}, gap = 0x{:X}, alloc_start = 0x{:X}",
        heap_start,
        heap_size,
        free_pages,
        gap,
        alloc_start
    );

    for i in 0..free_pages {
        unsafe { (*pages_entry.add(i)).clear() }
    }
    unsafe {
        ALLOC_PAGES_SUM = free_pages;
        ALLOC_START = alloc_start;
        ALLOC_END = ALLOC_START + ALLOC_PAGES_SUM * PAGE_SIZE;
        PAGES_ENTRY = pages_entry;
    };
}

// #[inline]
// unsafe fn heap_size() -> usize {
//     let mut val;
//     core::arch::asm!("la {0}, _heap_size", out(reg)val);
//     val
// }

// #[inline]
// unsafe fn heap_start() -> usize {
//     let mut val;
//     core::arch::asm!("la {0}, _heap_start", out(reg)val);
//     val
// }

// #[inline]
// unsafe fn pages_entry() -> *mut Page {
//     heap_start() as *mut Page
// }

/// allocate contiguous pages
pub fn alloc(pages: usize) -> *mut u8 {
    assert!(pages > 0);

    let mut founded = 0;
    let mut ptr = null_mut();
    unsafe {
        for i in 0..ALLOC_PAGES_SUM {
            // check, free ?
            if (*PAGES_ENTRY.add(i)).is_free() {
                founded += 1;
                // found contiguous pages, break and return the pointer
                if founded == pages {
                    // take them
                    for j in (i + 1 - founded)..i {
                        (*PAGES_ENTRY.add(j)).set_flag(PageBit::Taken);
                    }
                    // set the Last flag
                    (*PAGES_ENTRY.add(i)).set_flag(PageBit::Last);
                    // get mem pointer
                    ptr = (ALLOC_START + (i + 1 - founded) * PAGE_SIZE) as *mut u8;
                    break;
                }
                // not enough pages
                continue;
            }
            // it's not contiguous, so find again in the left
            founded = 0;
        }
    }
    ptr
}

pub fn dealloc(ptr: *mut u8) {
    assert!(!ptr.is_null());
    unsafe {}
}

use core::alloc::GlobalAlloc;

struct TestGlobalAlloc;

unsafe impl GlobalAlloc for TestGlobalAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut pages = layout.size() / 4096;
        if pages == 0 {
            pages = 1;
        }
        alloc(pages)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        assert!(!ptr.is_null());
        assert!((ptr as usize) >= ALLOC_START && (ptr as usize) < ALLOC_END);
        println!("ptr: 0x{:X}", ptr as usize);
        assert!((ptr as usize) & !(PAGE_SIZE - 1) == (ptr as usize));

        let mut index = (ptr as usize - ALLOC_START) / PAGE_SIZE;
        while let flag_ptr = &mut (*PAGES_ENTRY.add(index)).flag {
            match *flag_ptr {
                1 => *flag_ptr = PageBit::Free.val(),
                2 => {
                    *flag_ptr = PageBit::Free.val();
                    break;
                }
                0 => panic!("It shouldn't be a FreePage ! Double free ?"),
                _ => panic!("Unsupport phy-page flag"),
            }
            index += 1;
        }
    }
}

#[global_allocator]
static TEST_GLOBAL_ALLOC: TestGlobalAlloc = TestGlobalAlloc;
