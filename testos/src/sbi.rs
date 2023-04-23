const EID_SBI_CONSOLE_PUTCHAR: usize = 0x01;

const EID_SBI_DEBUG_CONSOLE: usize = 0x4442434E;
const FID_SBI_DEBUG_CONSOLE_WRITE: usize = 0;
const FID_SBI_DEBUG_CONSOLE_READ: usize = 1;
const FID_SBI_DEBUG_CONSOLE_WRITE_BYTE: usize = 2;

#[derive(PartialEq)]
pub enum SbiErr {
    // SbiSuccess = 0,
    SbiErrFailed = -1,
    SbiErrNotSupported = -2,
    SbiErrInvalidParam = -3,
    SbiErrDenied = -4,
    SbiErrInvalidAddress = -5,
    SbiErrAlreadyAvailable = -6,
    SbiErrAlreadyStarted = -7,
    SbiErrAlreadyStopped = -8,
}

#[inline]
fn sbi_result_usize(a0: usize, a1: usize) -> Result<usize, SbiErr> {
    match a0 as isize {
        0 => Ok(a1),
        -1 => Err(SbiErr::SbiErrFailed),
        -3 => Err(SbiErr::SbiErrInvalidParam),
        -4 => Err(SbiErr::SbiErrDenied),
        -5 => Err(SbiErr::SbiErrInvalidAddress),
        -6 => Err(SbiErr::SbiErrAlreadyAvailable),
        -7 => Err(SbiErr::SbiErrAlreadyStarted),
        -8 => Err(SbiErr::SbiErrAlreadyStopped),
        _ => Err(SbiErr::SbiErrNotSupported),
        // -2 => SbiErr::SbiErrNotSupported,
    }
}

#[inline]
fn sbi_result_none(a0: usize) -> Result<(), SbiErr> {
    match a0 as isize {
        0 => Ok(()),
        -1 => Err(SbiErr::SbiErrFailed),
        -3 => Err(SbiErr::SbiErrInvalidParam),
        -4 => Err(SbiErr::SbiErrDenied),
        -5 => Err(SbiErr::SbiErrInvalidAddress),
        -6 => Err(SbiErr::SbiErrAlreadyAvailable),
        -7 => Err(SbiErr::SbiErrAlreadyStarted),
        -8 => Err(SbiErr::SbiErrAlreadyStopped),
        _ => Err(SbiErr::SbiErrNotSupported),
        // -2 => SbiErr::SbiErrNotSupported,
    }
}

#[inline]
pub fn sbi_call_fast(eid: usize, fid: usize, arg0: usize, mut arg1: usize, arg2: usize) -> usize {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") arg0,
            inlateout("a1") arg1 => arg1,
            in("a2") arg2,
            in("a7") eid,
            in("a6") fid,
        )
    }
    arg1
}

#[inline]
pub fn sbi_call(
    eid: usize,
    fid: usize,
    mut arg0: usize,
    mut arg1: usize,
    arg2: usize,
) -> Result<usize, SbiErr> {
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => arg0,
            inlateout("a1") arg1 => arg1,
            in("a2") arg2,
            in("a7") eid,
            in("a6") fid,
        )
    }
    sbi_result_usize(arg0, arg1)
}

#[inline]
pub fn sbi_call_fast_none(
    eid: usize,
    fid: usize,
    mut arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<(), SbiErr> {
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a7") eid,
            in("a6") fid,
        )
    }
    sbi_result_none(arg0)
}

#[inline]
pub fn sbi_call4_fast_none(
    eid: usize,
    fid: usize,
    mut arg0: usize,
    arg1: usize,
    // arg2: usize,
) -> Result<(), SbiErr> {
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => arg0,
            in("a1") arg1,
            // in("a2") arg2,
            in("a7") eid,
            in("a6") fid,
        )
    }
    sbi_result_none(arg0)
}

#[inline]
pub fn sbi_call4_fast(eid: usize, fid: usize, mut arg0: usize, mut arg1: usize) -> usize {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") arg0,
            inlateout("a1") arg1 => arg1,
            // in("a2") arg2,
            in("a7") eid,
            in("a6") fid,
        )
    }
    arg1
}

#[inline]
pub fn sbi_call4(
    eid: usize,
    fid: usize,
    mut arg0: usize,
    mut arg1: usize,
) -> Result<usize, SbiErr> {
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => arg0,
            inlateout("a1") arg1 => arg1,
            // in("a2") arg2,
            in("a7") eid,
            in("a6") fid,
        )
    }
    sbi_result_usize(arg0, arg1)
}

#[inline]
pub fn sbi_debug_console_write(
    num_bytes: usize,
    base_addr_lo: usize,
    base_addr_hi: usize,
) -> Result<(), SbiErr> {
    sbi_call_fast_none(
        EID_SBI_DEBUG_CONSOLE,
        FID_SBI_DEBUG_CONSOLE_WRITE_BYTE,
        num_bytes as usize,
        base_addr_lo,
        base_addr_hi,
    )
}


#[inline]// blocking until return
pub fn sbi_debug_console_write_byte(byte: u8) -> Result<(), SbiErr> {
    sbi_call4_fast_none(
        EID_SBI_DEBUG_CONSOLE,
        FID_SBI_DEBUG_CONSOLE_WRITE_BYTE,
        byte as usize,
        0,
    )
}


#[inline]// blocking until return
pub fn sbi_debug_console_write_byte_fast(byte: u8) {
    sbi_call4_fast(
        EID_SBI_DEBUG_CONSOLE,
        FID_SBI_DEBUG_CONSOLE_WRITE_BYTE,
        byte as usize,
        0,
    );
}
