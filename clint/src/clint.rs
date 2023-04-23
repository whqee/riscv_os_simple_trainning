// CLint base_addr: 0x0200_0000
// CLint MTime: base_addr + 0xbff8
// CLint MTimeCMP: base_addr + 0x4000

use crate::cpu::mhartid;

const CLINT_BASE_ADDR: usize = 0x0200_0000;
const CLINT_MTIME_ADDR: usize = CLINT_BASE_ADDR + 0xBFF8;
const CLINT_MTIMECMP_ADDR: usize = CLINT_BASE_ADDR + 0x4000;

pub(crate) struct CLint;
impl CLint {
    #[inline]
    pub fn get_mtime() -> u64 {
        unsafe { (CLINT_MTIME_ADDR as *mut u64).read_volatile() }
    }

    #[inline]
    pub fn set_timer(time_val: u64) {
        unsafe {
            (CLINT_MTIMECMP_ADDR as *mut u64).add(mhartid())
                    .write_volatile(time_val);
        }
    }
}


