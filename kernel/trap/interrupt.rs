#[cfg(target_arch = "riscv64")]
pub fn interrupt_on() {}

#[cfg(target_arch = "riscv64")]
pub fn interrupt_off() {}

#[cfg(target_arch = "riscv64")]
fn s_timer_int() {}

#[cfg(target_arch = "riscv64")]
fn s_extern_int() {}

#[cfg(target_arch = "riscv64")]
fn s_software_int() {}

pub fn instr_on() {}

pub fn instr_off() {}
