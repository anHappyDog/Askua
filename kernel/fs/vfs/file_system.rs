use alloc::sync::Arc;

use crate::{driver::virtio::blk::VirtioBlkDevice, lock::spin::Spinlock};

struct FileSystem<'a> {
    name: &'a str,
    device: Arc<Spinlock<dyn VirtioBlkDevice>>,
}
