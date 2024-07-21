pub mod context;
pub mod interrupt;
pub mod syscall;
use crate::arch::rv64::reg::sys::Scause;

use crate::arch::rv64::{self, reg::Reg};
use crate::printk;

#[cfg(target_arch = "riscv64")]
fn store_access_fault(context: &mut context::UserContext) {}

#[cfg(target_arch = "riscv64")]
fn store_address_misaligned(context: &mut context::UserContext) {}

#[cfg(target_arch = "riscv64")]
fn load_access_fault(context: &mut context::UserContext) {}

#[cfg(target_arch = "riscv64")]
fn load_address_misaligned(context: &mut context::UserContext) {}

#[cfg(target_arch = "riscv64")]
fn breakpoint(context: &mut context::UserContext) {}

#[cfg(target_arch = "riscv64")]
fn illegal_instruction(context: &mut context::UserContext) {}

#[cfg(target_arch = "riscv64")]
fn instruction_access_fault(context: &mut context::UserContext) {}

#[cfg(target_arch = "riscv64")]
fn instruction_address_misaligned(context: &mut context::UserContext) {}

#[cfg(target_arch = "riscv64")]
fn reserved_trap(context: &mut context::UserContext) {}

extern "C" {
    fn kernelvec();
}

#[cfg(target_arch = "riscv64")]
pub(super) fn init() {
    rv64::reg::sys::Stvec::write(kernelvec as usize);
}

#[cfg(target_arch = "riscv64")]
#[no_mangle]
extern "C" fn ktrap(context: &mut context::KernelContext) {
    if Scause::is_instr() {
        interrupt::kinstr(context, Scause::exc_code());
    } else {
        ktrap_handler(context, Scause::exc_code());
    }
}

fn ktrap_handler(context: &mut context::KernelContext, cause: usize) {
    panic!("Kernel trap handler: cause = {}\n", cause);
}

#[cfg(target_arch = "riscv64")]
#[no_mangle]
extern "C" fn utrap(context: &mut context::UserContext) {}

#[cfg(target_arch = "riscv64")]
#[no_mangle]
extern "C" fn urettrap(context: &mut context::UserContext) {}
