use core::ptr::null_mut;

use crate::println;

pub const PAGE_SIZE: usize = 4096;
static mut ALLOC_START: usize = 0;
static mut ALLOC_END: usize = 0;
static mut PAGES_SUM_ALLOC: usize = 0;
static mut PAGES_ENTRY: *mut Page = null_mut();

/// 1) is this page taken? and
/// 2) is this the last page of a contiguous allocation? (when we free them)
#[repr(u8)]
enum PageBit {
    Free,
    Taken = 1 << 0,
    Last = 1 << 1,
}

struct Page {
    flag: u8,
}

impl PageBit {
    pub fn val(self) -> u8 {
        self as u8
    }
}

impl From<u8> for PageBit {
    fn from(val: u8) -> Self {
        match val {
            0 => PageBit::Free,
            1 => PageBit::Taken,
            2 => PageBit::Last,
            _ => panic!("Unsupported Page Bit"),
        }
    }
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
        PAGES_SUM_ALLOC = free_pages;
        ALLOC_START = alloc_start;
        ALLOC_END = ALLOC_START + PAGES_SUM_ALLOC * PAGE_SIZE;
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

#[inline]
fn page_index(index: usize) -> &'static mut Page {
    unsafe { &mut *PAGES_ENTRY.add(index) }
}

/// allocate contiguous pages
pub unsafe fn alloc(pages: usize) -> *mut u8 {
    assert!(pages > 0);

    if pages > unsafe { PAGES_SUM_ALLOC } {
        println!("No enough pages={:X} to be alloc", pages);
        return null_mut();
    }

    let mut founded = 0;
    let mut ptr = null_mut();
    unsafe {
        for i in 0..PAGES_SUM_ALLOC {
            // check, free ?
            if page_index(i).is_free() {
                founded += 1;
                // found contiguous pages, break and return the pointer
                if founded == pages {
                    // take them
                    for j in (i + 1 - founded)..i {
                        page_index(j).set_flag(PageBit::Taken);
                    }
                    // set the Last flag
                    page_index(i).set_flag(PageBit::Last);
                    // get mem pointer
                    ptr = (ALLOC_START + (i + 1 - founded) * PAGE_SIZE) as *mut u8;
                    println!(
                        "Alloc {} page(s): [0x{:X}-0x{:X})",
                        pages,
                        ptr as usize,
                        ptr as usize + pages * 4096
                    );
                    // break;
                    return ptr;
                }
                // not enough pages
                continue;
            }
            // it's not contiguous, so find again in the left
            founded = 0;
        }
    }
    println!("No enough free pages");
    ptr
}

pub unsafe fn zalloc(pages: usize) -> *mut u8 {
    let ptr = alloc(pages);
    if ptr.is_null() {
        ptr
    } else {
        // Clear the Pages
        unsafe { core::slice::from_raw_parts_mut(ptr, PAGE_SIZE).fill(0) }
        ptr
    }
}

pub unsafe fn dealloc(ptr: *mut u8) {
    assert!(!ptr.is_null());
    println!("dealloc 0x{:X}", ptr as usize);
    assert!((ptr as usize) & !(PAGE_SIZE - 1) == (ptr as usize));
    unsafe {
        assert!((ptr as usize) >= ALLOC_START && (ptr as usize) < ALLOC_END);

        let index = (ptr as usize - ALLOC_START) / PAGE_SIZE;

        for i in index..PAGES_SUM_ALLOC {
            let flag = &mut page_index(i).flag;

            match (*flag).into() {
                PageBit::Taken => *flag = PageBit::Free.val(),
                PageBit::Last => {
                    *flag = PageBit::Free.val();
                    println!(
                        "Free {} page(s): [0x{:X}-0x{:X})",
                        i - index + 1,
                        ptr as usize,
                        ptr as usize + (i - index) * 4096 + 4096
                    );
                    break;
                }
                PageBit::Free => panic!("Trying to free a FreePage ! Double free ?"),
            }
        }
    }
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
        dealloc(ptr)
    }
}

#[global_allocator]
static TEST_GLOBAL_ALLOC: TestGlobalAlloc = TestGlobalAlloc;
