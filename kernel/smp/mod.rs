use alloc::{sync::Arc, vec::Vec};

use crate::lock::irq_safe::spin::IrqSafeSpinlock;

pub struct Cpu {}

impl Cpu {}

pub fn current_cpu_id() -> usize {
    0
}


pub static CPU_LIST: IrqSafeSpinlock<Vec<Arc<IrqSafeSpinlock<Cpu>>>> =
    IrqSafeSpinlock::new(Vec::new());


