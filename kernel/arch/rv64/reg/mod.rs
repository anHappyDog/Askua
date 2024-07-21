pub mod gpr;
pub mod sys;

pub trait Reg {
    fn read() -> usize;
    fn write(value: usize);
}
