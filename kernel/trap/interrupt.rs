use super::context;

#[cfg(target_arch = "riscv64")]
fn u_timer_int() {}

#[cfg(target_arch = "riscv64")]
fn u_extern_int() {}

#[cfg(target_arch = "riscv64")]
fn u_software_int() {}
#[cfg(target_arch = "riscv64")]
pub fn instr_on() {}
#[cfg(target_arch = "riscv64")]
pub fn instr_off() {}
#[cfg(target_arch = "riscv64")]
pub fn timer_instr_on() {}
#[cfg(target_arch = "riscv64")]
pub fn timer_instr_off() {}
#[cfg(target_arch = "riscv64")]
fn ks_software_int() {}
#[cfg(target_arch = "riscv64")]
fn ks_timer_int() {}
#[cfg(target_arch = "riscv64")]
fn ks_extern_int() {}

const S_SOFTWARE_INSTR: usize = 1;
const S_TIMER_INSTR: usize = 5;
const S_EXTERNAL_INSTR: usize = 9;
const S_COUNTER_OVERFLOW_INSTR: usize = 13;

pub(super) fn kinstr(context: &mut context::KernelContext, cause: usize) {
    match cause {
        S_SOFTWARE_INSTR => {
            ks_software_int();
        }
        S_TIMER_INSTR => {
            ks_timer_int();
        }
        S_EXTERNAL_INSTR => {
            ks_extern_int();
        }
        S_COUNTER_OVERFLOW_INSTR => {
            panic!("Counter overflow interrupt in kernel\n");
        }
        _ => {
            panic!("Unknown interrupt in kernel: cause = {}\n", cause);
        }
    }
}

pub(super) fn uinstr(context: &mut context::UserContext, cause: usize) {
    match cause {
        S_SOFTWARE_INSTR => {
            u_software_int();
        }
        S_TIMER_INSTR => {
            u_timer_int();
        }
        S_EXTERNAL_INSTR => {
            u_extern_int();
        }
        S_COUNTER_OVERFLOW_INSTR => {
            panic!("Counter overflow interrupt in user\n");
        }
        _ => {
            panic!("Unknown interrupt in user: cause = {}\n", cause);
        }
    }
}
