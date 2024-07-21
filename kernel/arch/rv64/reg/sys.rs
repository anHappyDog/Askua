use super::Reg;
use core::arch::asm;

macro_rules! DEFINE_SYSREG {
    ($name:ident, $num:expr) => {
        pub struct $name;

        impl Reg for $name {
            fn read() -> usize {
                let value: usize;
                unsafe {
                    asm!(
                        "csrr {}, {}",
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
                        "csrw {}, {}",
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

DEFINE_SYSREG!(Sstatus, 0x100);
DEFINE_SYSREG!(Sedeleg, 0x102);
DEFINE_SYSREG!(Sideleg, 0x103);
DEFINE_SYSREG!(Sie, 0x104);
DEFINE_SYSREG!(Stvec, 0x105);
DEFINE_SYSREG!(Scounteren, 0x106);
DEFINE_SYSREG!(Sscratch, 0x140);
DEFINE_SYSREG!(Sepc, 0x141);
DEFINE_SYSREG!(Scause, 0x142);
DEFINE_SYSREG!(Stval, 0x143);
DEFINE_SYSREG!(Sip, 0x144);
