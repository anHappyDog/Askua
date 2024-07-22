use alloc::{boxed::Box, sync::Arc};

use crate::lock::irq_safe::spin::IrqSafeSpinlock;

use super::{dentry::Dentry, inode::Inode};

pub trait File {}

struct Kstat {}
