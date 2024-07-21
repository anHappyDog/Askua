use super::Reg;
use core::arch::asm;
macro_rules! DEFINE_GPR {
    ($name:ident, $num:expr) => {
        pub struct $name;

        impl Reg for $name {
            fn read() -> usize {
                let value: usize;
                unsafe {
                    asm!(
                        "mv {}, x{}",
                        out(reg) value,
                        const $num,
                        options(nostack)
                    );
                }
                value
            }

            fn write(value: usize) {
                unsafe {
                    asm!(
                        "mv x{}, {}",
                        in(reg) value,
                        const $num,
                        options(nostack)
                    );
                }
            }
        }
    };
    () => {};
}


DEFINE_GPR!(Ra, 1);
DEFINE_GPR!(Sp, 2);
DEFINE_GPR!(Gp, 3);
DEFINE_GPR!(Tp, 4);
DEFINE_GPR!(T0, 5);
DEFINE_GPR!(T1, 6);
DEFINE_GPR!(T2, 7);
DEFINE_GPR!(S0, 8);
DEFINE_GPR!(S1, 9);
DEFINE_GPR!(A0, 10);
DEFINE_GPR!(A1, 11);
DEFINE_GPR!(A2, 12);
DEFINE_GPR!(A3, 13);
DEFINE_GPR!(A4, 14);
DEFINE_GPR!(A5, 15);
DEFINE_GPR!(A6, 16);
DEFINE_GPR!(A7, 17);
DEFINE_GPR!(S2, 18);
DEFINE_GPR!(S3, 19);
DEFINE_GPR!(S4, 20);
DEFINE_GPR!(S5, 21);
DEFINE_GPR!(S6, 22);
DEFINE_GPR!(S7, 23);
DEFINE_GPR!(S8, 24);
DEFINE_GPR!(S9, 25);
DEFINE_GPR!(S10, 26);
DEFINE_GPR!(S11, 27);
DEFINE_GPR!(T3, 28);
DEFINE_GPR!(T4, 29);
DEFINE_GPR!(T5, 30);
DEFINE_GPR!(T6, 31);
