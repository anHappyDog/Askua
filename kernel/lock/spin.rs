use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Spinlock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Spinlock<T> {}

unsafe impl<T> Send for Spinlock<T> {}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Spinlock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinlockGuard<T> {
        // mips32::int::disable_timer_interrupt();
        loop {
            match self
                .lock
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            {
                Ok(_) => break,
                Err(_) => {
                    core::hint::spin_loop();
                }
            }
        }
        // mips32::int::enable_timer_interrupt();
        SpinlockGuard { lock: self }
    }

    pub fn try_lock(&self) -> Option<SpinlockGuard<T>> {
        match self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
        {
            Ok(_) => Some(SpinlockGuard { lock: self }),
            Err(_) => None,
        }
    }
}

pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

impl<'a, T> Drop for SpinlockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.lock.store(false, Ordering::Release);
    }
}

impl<'a, T> core::ops::Deref for SpinlockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}
