use alloc::{boxed::Box, vec::Vec};

pub(super) const VIRTQ_DESC_LIST_LENGTH: usize = 256;

#[repr(C, align(4096))]
pub(super) struct Virtq {
    desc_list: [VirtqDesc; VIRTQ_DESC_LIST_LENGTH],
    avail: VirtqAvail,
    used: VirtqUsed,
    bitmap: [u8; (VIRTQ_DESC_LIST_LENGTH + 7) / 8],
}





impl Virtq {
    pub fn get_desc_idx(&self) -> u16 {
        self.avail.idx
    }
    pub fn get_used_idx(&self) -> u16 {
        self.used.idx
    }
    pub fn increment_idx(&mut self) {
        self.avail.idx += 1;
    }
    pub fn fill_avail(&mut self,index : u16) {
        let idx = self.avail.idx;
        self.avail.ring[idx as usize % VIRTQ_DESC_LIST_LENGTH] = index;
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        self.avail.idx += 1;
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    }
    pub fn init() -> Box<Self> {
        Box::new(Self {
            desc_list: core::array::from_fn::<_, VIRTQ_DESC_LIST_LENGTH, _>(|_| {
                VirtqDesc::default()
            }),
            avail: VirtqAvail::init(),
            used: VirtqUsed::init(),
            bitmap: [0; (VIRTQ_DESC_LIST_LENGTH + 7) / 8],
        })
    }
    pub fn desc_alloc(&mut self) -> Option<(u16)> {
        for i in 0..((VIRTQ_DESC_LIST_LENGTH + 7) / 8) {
            if self.bitmap[i / 8] & (1 << (i % 8)) == 0 {
                self.bitmap[i / 8] |= 1 << (i % 8);
                return Some(i as u16);
            }
        }
        None
    }
    pub fn get_desc(&self, index: u16) -> &VirtqDesc {
        &self.desc_list[index as usize]
    }

    pub fn get_desc_mut(&mut self, index: u16) -> &mut VirtqDesc {
        &mut self.desc_list[index as usize]
    }

    pub fn desc_free(&mut self, index: usize) {
        self.bitmap[index / 8] &= !(1 << (index % 8));
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
#[derive(Default)]
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
    pub fn set_addr(&mut self, addr: u64) {
        self.addr = addr;
    }
    pub fn set_len(&mut self, len: u32) {
        self.len = len;
    }
    pub fn set_flags(&mut self, flags: u16) {
        self.flags = flags;
    }
    pub fn set_next(&mut self, next: u16) {
        self.next = next;
    }
}

#[repr(C, align(2))]
pub(super) struct VirtqAvail {
    flags: u16,
    idx: u16,
    ring: [u16; VIRTQ_DESC_LIST_LENGTH],
    used_event: u16,
}

impl VirtqAvail {
    fn init() -> Self {
        Self {
            flags: 0,
            idx: 0,
            ring: [0; VIRTQ_DESC_LIST_LENGTH],
            used_event: 0,
        }
    }
}

#[repr(C, packed)]
#[derive(Default)]
pub(super) struct VirtqUsedElem {
    id: u32,
    len: u32,
}

#[repr(C, align(4096))]
pub(super) struct VirtqUsed {
    flags: u16,
    idx: u16,
    ring: [VirtqUsedElem; VIRTQ_DESC_LIST_LENGTH],
    avail_event: u16,
}

impl VirtqUsed {
    fn init() -> Self {
        Self {
            flags: 0,
            idx: 0,
            ring: core::array::from_fn::<_, VIRTQ_DESC_LIST_LENGTH, _>(|_| {
                VirtqUsedElem::default()
            }),
            avail_event: 0,
        }
    }
}

pub(super) const VIRTQ_USED_F_NO_NOTIFY: u32 = 1;
pub(super) const VIRTQ_AVAIL_F_NO_INTERRUPT: u16 = 1;
pub(super) const VIRTQ_DESC_F_NEXT: u16 = 1;
pub(super) const VIRTQ_DESC_F_WRITE: u16 = 2;
pub(super) const VIRTQ_DESC_F_INDERECT: u32 = 4;
