use core::ops::Add;

use super::Rtc;
use crate::{driver::Device, lock::irq_safe::spin::IrqSafeSpinlock};
pub struct Goldfish {
    base: usize,
    size: usize,
}

const GOLDFISH_RTC_BASE: usize = 0x101000;
const GOLDFISH_RTC_SIZE: usize = 0x1000;

const GOLDFISH_RTC_TIME_LOW: usize = 0x0;
const GOLDFISH_RTC_TIME_HIGH: usize = 0x4;
const GOLDFISH_RTC_ALARM_LOW: usize = 0x8;
const GOLDFISH_RTC_ALARM_HIGH: usize = 0xc;
const GOLDFISH_RTC_IRQ_ENABLE: usize = 0x10;
const GOLDFISH_RTC_ALARM_STATUS: usize = 0x18;

lazy_static::lazy_static! {
    pub static ref GOLDFISH_RTC : IrqSafeSpinlock<Goldfish> = IrqSafeSpinlock::new(Goldfish::init(GOLDFISH_RTC_BASE, GOLDFISH_RTC_SIZE));
}

impl Rtc for Goldfish {
    fn init(base: usize, size: usize) -> Self {
        Goldfish { base, size }
    }

    fn read_time(&self) -> u64 {
        let low: u32 = self.read_volatile(GOLDFISH_RTC_TIME_LOW);
        let high: u32 = self.read_volatile(GOLDFISH_RTC_TIME_HIGH);
        ((high as u64) << 32) | (low as u64)
    }

    fn write_time(&self, time: u64) {
        let low = time as u32;
        let high = (time >> 32) as u32;
        self.write_volatile(GOLDFISH_RTC_TIME_LOW, low);
        self.write_volatile(GOLDFISH_RTC_TIME_HIGH, high);
    }

    fn read_alarm(&self) -> u64 {
        let low: u32 = self.read_volatile(GOLDFISH_RTC_ALARM_LOW);
        let high: u32 = self.read_volatile(GOLDFISH_RTC_ALARM_HIGH);
        ((high as u64) << 32) | (low as u64)
    }

    fn irq_is_enabled(&self) -> bool {
        self.read_volatile::<u32>(GOLDFISH_RTC_IRQ_ENABLE) != 0
    }

    fn enable_irq(&self) {
        todo!()
    }

    fn disable_irq(&self) {
        todo!()
    }

    fn alarm_status(&self) -> u32 {
        self.read_volatile(GOLDFISH_RTC_ALARM_STATUS)
    }

    fn clear_alarm(&self) {
        todo!()
    }
}

impl Device for Goldfish {
    fn read_volatile<T>(&self, offset: usize) -> T
    where
        T: Add,
    {
        unsafe { ((self.base + offset) as *const T).read_volatile() }
    }

    fn write_volatile<T>(&self, offset: usize, value: T) {
        unsafe {
            ((self.base + offset) as *mut T).write_volatile(value);
        }
    }
}
