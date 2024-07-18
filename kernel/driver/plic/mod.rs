pub mod sifive;

pub trait Plic {
    fn init(base: usize, size: usize) -> Self;
    fn claim(&self, context: usize) -> u32;
    fn complete(&self, context: usize, irq: u32);
}
