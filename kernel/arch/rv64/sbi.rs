use super::reg::sys::ecall;

pub struct SbiRet {
    error: i64,
}

// v0.1 legacy
pub const SBI_SET_TIMER: usize = 0;
pub const SBI_CONSOLE_PUTCHAR: usize = 1;
pub const SBI_CONSOLE_GETCHAR: usize = 2;
pub const SBI_CLEAR_IPI: usize = 3;
pub const SBI_SEND_IPI: usize = 4;
pub const SBI_REMOTE_FENCE_I: usize = 5;
pub const SBI_REMOTE_SFENCE_VMA: usize = 6;
pub const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
pub const SBI_SHUTDOWN: usize = 8;

// v0.2 and higher
pub const SBI_EXT_BASE: usize = 0x10;
pub const SBI_EXT_TIME: usize = 0x54494D45;
pub const SBI_EXT_IPI: usize = 0x735049;
pub const SBI_EXT_RFENCE: usize = 0x52464E43;
pub const SBI_EXT_HSM: usize = 0x48534D;
pub const SBI_EXT_SRST: usize = 0x53525354;
pub const SBI_EXT_PMU: usize = 0x504D55;
pub const SBI_EXT_DBCN: usize = 0x4442434E;
pub const SBI_EXT_SUSP: usize = 0x53555350;
pub const SBI_EXT_CPPC: usize = 0x43505043;
pub const SBI_EXT_DBTR: usize = 0x44425452;
pub const SBI_EXT_SSE: usize = 0x535345;

pub const SBI_EXT_IPI_SEND_IPI: usize = 0x0;

/* SBI return error codes */
pub const SBI_SUCCESS: isize = 0;
pub const SBI_ERR_FAILED: isize = -1;
pub const SBI_ERR_NOT_SUPPORTED: isize = -2;
pub const SBI_ERR_INVALID_PARAM: isize = -3;
pub const SBI_ERR_DENIED: isize = -4;
pub const SBI_ERR_INVALID_ADDRESS: isize = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: isize = -6;
pub const SBI_ERR_ALREADY_STARTED: isize = -7;
pub const SBI_ERR_ALREADY_STOPPED: isize = -8;
pub const SBI_ERR_NO_SHMEM: isize = -9;
pub const SBI_ERR_INVALID_STATE: isize = -10;
pub const SBI_ERR_BAD_RANGE: isize = -11;

// sbi functions for hsm extension
const SBI_EXT_HSM_HART_START: usize = 0x0;
const SBI_EXT_HSM_HART_STOP: usize = 0x1;
const SBI_EXT_HSM_HART_GET_STATUS: usize = 0x2;
const SBI_EXT_HSM_HART_SUSPEND: usize = 0x3;

pub fn sbi_hart_start(hartid: usize, start_addr: usize, opaque: usize) -> SbiRet {
    let error = ecall(hartid, start_addr, opaque, 0, 0, 0, SBI_EXT_HSM_HART_START) as i64;
    SbiRet { error }
}

pub fn sbi_hart_stop() -> SbiRet {
    let error = ecall(0, 0, 0, 0, 0, 0, SBI_EXT_HSM_HART_STOP) as i64;
    SbiRet { error }
}

pub fn sbi_hart_suspend(suspend_type: usize, resume_addr: usize, opaque: usize) -> SbiRet {
    // Assuming you will fill in the details later
    SbiRet { error: 0 }
}

pub fn sbi_hart_wakeup(hartid: usize) -> SbiRet {
    let error = ecall(hartid, 0, 0, 0, 0, 0, SBI_EXT_HSM_HART_SUSPEND) as i64;
    SbiRet { error }
}

pub fn sbi_send_ipi(hart_mask: usize, hart_mask_base: usize) -> SbiRet {
    let error = ecall(hart_mask, hart_mask_base, 0, 0, 0, 0, SBI_EXT_IPI_SEND_IPI) as i64;
    SbiRet { error }
}

pub fn sbi_set_timer(stime_value: usize) -> SbiRet {
    let error = ecall(stime_value, 0, 0, 0, 0, 0, SBI_SET_TIMER) as i64;
    SbiRet { error }
}

pub fn sbi_hart_get_status(hartid: usize) -> SbiRet {
    let error = ecall(hartid, 0, 0, 0, 0, 0, SBI_EXT_HSM_HART_GET_STATUS) as i64;
    SbiRet { error }
}

pub fn sbi_shutdown() -> !  {
    ecall(0, 0, 0, 0, 0, 0, SBI_SHUTDOWN) as i64;
    unreachable!("sbi_shutdown failed");
}

pub fn sbi_clear_ipi() {
    ecall(0, 0, 0, 0, 0, 0, SBI_CLEAR_IPI) as i64;
}
