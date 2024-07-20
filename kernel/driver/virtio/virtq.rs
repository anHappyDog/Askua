use alloc::vec::Vec;

pub(super) const VIRTQ_DESC_LIST_LENGTH: usize = 4;

#[repr(C, align(4096))]
pub(super) struct Virtq {
    desc_list: [VirtqDesc; VIRTQ_DESC_LIST_LENGTH],
    avail: VirtqAvail,
    used: VirtqUsed,
    bitmap: [u8; (VIRTQ_DESC_LIST_LENGTH + 7) / 8],
}

impl Virtq {
    pub fn init() -> Self {
        Self {
            desc_list: [VirtqDesc::init(); VIRTQ_DESC_LIST_LENGTH],
            avail: VirtqAvail::init(),
            used: VirtqUsed::init(),
            bitmap: [0; (VIRTQ_DESC_LIST_LENGTH + 7) / 8],
        }
    }
    pub fn desc_addr(&self) -> usize {
        &self.desc_list as *const _ as usize
    }
    pub fn avail_addr(&self) -> usize {
        &self.avail as *const _ as usize
    }
    pub fn used_addr(&self) -> usize {
        &self.used as *const _ as usize
    }
    pub fn set_avail_flags(&mut self, flags: u16) {
        self.avail.flags = flags;
    }
    pub fn self_addr(&self) -> usize {
        self as *const _ as usize
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub(super) struct VirtqDesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

impl VirtqDesc {
    fn init() -> Self {
        Self {
            addr: 0,
            len: 0,
            flags: 0,
            next: 0,
        }
    }
}

const VIRTQ_QUEUE_SIZE: usize = 256;

#[repr(C, align(2))]
pub(super) struct VirtqAvail {
    flags: u16,
    idx: u16,
    ring: [u16; VIRTQ_QUEUE_SIZE],
    used_event: u16,
}

impl VirtqAvail {
    fn init() -> Self {
        Self {
            flags: 0,
            idx: 0,
            ring: [0; VIRTQ_QUEUE_SIZE],
            used_event: 0,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct VirtqUsedElem {
    id: u32,
    len: u32,
}

#[repr(C, align(4096))]
pub(super) struct VirtqUsed {
    flags: u16,
    idx: u16,
    ring: [VirtqUsedElem; VIRTQ_QUEUE_SIZE],
    avail_event: u16,
}

impl VirtqUsed {
    fn init() -> Self {
        Self {
            flags: 0,
            idx: 0,
            ring: [VirtqUsedElem { id: 0, len: 0 }; VIRTQ_QUEUE_SIZE],
            avail_event: 0,
        }
    }
}

pub(super) const VIRTQ_USED_F_NO_NOTIFY: u32 = 1;
pub(super) const VIRTQ_AVAIL_F_NO_INTERRUPT: u32 = 1;
pub(super) const VIRTQ_DESC_F_NEXT: u32 = 1;
pub(super) const VIRTQ_DESC_F_WRITE: u32 = 2;
pub(super) const VIRTQ_DESC_F_INDERECT: u32 = 4;

