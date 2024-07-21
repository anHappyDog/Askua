use alloc::sync::Arc;

use crate::{
    driver::virtio::blk::VirtioBlkDevice,
    lock::{irq_safe::spin::IrqSafeSpinlock},
};

struct FileSystem<'a> {
    name: &'a str,
    device: Arc<IrqSafeSpinlock<dyn VirtioBlkDevice>>,
}
