use crate::arch::rv64::reg::sys::Sstatus;
use crate::arch::rv64::reg::Reg;
use core::{cell::UnsafeCell, sync::atomic::AtomicBool};
pub struct IrqSafeSpinlock<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct IrqSafeSpinlockGuard<'a, T> {
    lock: &'a IrqSafeSpinlock<T>,
    instr: usize,
}

unsafe impl<T> Sync for IrqSafeSpinlock<T> {}

unsafe impl<T> Send for IrqSafeSpinlock<T> {}

impl<T> IrqSafeSpinlock<T> {
    pub const fn new(data: T) -> Self {
        IrqSafeSpinlock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
    pub fn lock(&self) -> IrqSafeSpinlockGuard<T> {
        let instr = Sstatus::read();
        Sstatus::clear_sie();
        loop {
            match self.lock.compare_exchange(
                false,
                true,
                core::sync::atomic::Ordering::Acquire,
                core::sync::atomic::Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(_) => {
                    core::hint::spin_loop();
                }
            }
        }
        IrqSafeSpinlockGuard { lock: self, instr }
    }
    pub fn try_lock(&self) -> Option<IrqSafeSpinlockGuard<T>> {
        match self.lock.compare_exchange(
            false,
            true,
            core::sync::atomic::Ordering::Acquire,
            core::sync::atomic::Ordering::Acquire,
        ) {
            Ok(_) => {
                let instr = Sstatus::read();
                Sstatus::clear_sie();
                Some(IrqSafeSpinlockGuard { lock: self, instr })
            }
            Err(_) => None,
        }
    }
}

impl<'a, T> Drop for IrqSafeSpinlockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock
            .lock
            .store(false, core::sync::atomic::Ordering::Release);
        Sstatus::write(self.instr);
    }
}

impl<'a, T> core::ops::Deref for IrqSafeSpinlockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for IrqSafeSpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}
