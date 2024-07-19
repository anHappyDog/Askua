pub trait Address {}

pub struct VirtualAddress {
    address: usize,
}

pub struct PhysicalAddress {
    address: usize,
}

impl Address for VirtualAddress {}
impl Address for PhysicalAddress {}

impl From<usize> for VirtualAddress {
    fn from(address: usize) -> Self {
        VirtualAddress { address }
    }
}

impl From<usize> for PhysicalAddress {
    fn from(address: usize) -> Self {
        PhysicalAddress { address }
    }
}

pub struct UserObj<T> {
    pub inner: *mut T,
}

pub struct KernelObj<T> {
    pub inner: *mut T,
}
