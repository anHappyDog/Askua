use crate::{
    driver::uart::{ns16550a::Ns16550a, Uart},
    lock::spin::Spinlock,
};
use core::fmt::{self, Write};
const NS16550A_BASE: usize = 0x10000000;

pub struct Stdout;

lazy_static::lazy_static! {
    static ref NS16550A : Spinlock<Ns16550a> = Spinlock::new(Ns16550a::init(NS16550A_BASE, 0));
    static ref STDOUT : Spinlock<Stdout> = Spinlock::new(Stdout);
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            NS16550A.lock().putc(c as u32);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    STDOUT.lock().write_fmt(args).unwrap();
}


macro_rules! print {
    ($($arg:tt)*) => {
        $crate::log::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! printk {
    ($fmt:expr) => {
        $crate::log::_print(format_args!($fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::log::print(format_args!(concat!($fmt, "\n"), $($arg)*));
    };
}

#[macro_export]
macro_rules! log {
    () => {};
}

#[macro_export]
macro_rules! error {
    () => {};
}

#[macro_export]
macro_rules! warning {
    () => {};
}
