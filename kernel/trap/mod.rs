pub mod context;
pub mod interrupt;
pub mod syscall;

#[cfg(target_arch = "riscv64")]
fn store_access_fault(context: &mut context::Context) {}

#[cfg(target_arch = "riscv64")]
fn store_address_misaligned(context: &mut context::Context) {}

#[cfg(target_arch = "riscv64")]
fn load_access_fault(context: &mut context::Context) {}

#[cfg(target_arch = "riscv64")]
fn load_address_misaligned(context: &mut context::Context) {}

#[cfg(target_arch = "riscv64")]
fn breakpoint(context: &mut context::Context) {}

#[cfg(target_arch = "riscv64")]
fn illegal_instruction(context: &mut context::Context) {}

#[cfg(target_arch = "riscv64")]
fn instruction_access_fault(context: &mut context::Context) {}

#[cfg(target_arch = "riscv64")]
fn instruction_address_misaligned(context: &mut context::Context) {}

#[cfg(target_arch = "riscv64")]
fn reserved_trap(context: &mut context::Context) {}

#[cfg(target_arch = "riscv64")]
fn trap_init() {}

#[no_mangle]
extern "C" fn trap_handler(context: &mut context::Context) {
    
}
