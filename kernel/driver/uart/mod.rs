pub mod ns16550a;

pub trait Uart {
    fn getc(&self) -> u32;
    fn putc(&self, c: u32);
    fn init(base: usize, size: usize) -> Self;
}
