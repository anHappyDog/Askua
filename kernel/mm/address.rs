use core::ops::{Deref, DerefMut};

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

#[repr(C, packed)]
pub struct UserObj<T> {
    pub inner: *mut T,
}

pub trait KernelObjable {}



#[repr(C, packed)]
pub struct KernelObj<T>
where
    T: KernelObjable,
{
    pub inner: *mut T,
}

impl<T> Deref for KernelObj<T>
where
    T: KernelObjable,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.inner) as &T }
    }
}

impl<T> DerefMut for KernelObj<T>
where
    T: KernelObjable,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.inner) as &mut T }
    }
}
