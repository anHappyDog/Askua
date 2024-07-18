pub mod goldfish;

pub trait Rtc {
    fn init(base : usize, size : usize) -> Self;
    fn read_time(&self) -> u64;
    fn write_time(&self, time : u64);
    fn read_alarm(&self) -> u64;
    fn irq_is_enabled(&self) -> bool;
    fn enable_irq(&self);
    fn disable_irq(&self);
    fn alarm_status(&self) -> u32;
    fn clear_alarm(&self);
}