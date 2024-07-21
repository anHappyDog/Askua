use super::Reg;
use core::arch::asm;
macro_rules! DEFINE_SYSREG {
    ($name:ident, $csr_name:expr) => {
        pub struct $name;

        impl Reg for $name {
            
            #[inline(always)]
            fn read() -> usize {
                let value: usize;
                unsafe {
                    asm!(
                        concat!("csrr {}, ", $csr_name),
                        out(reg) value,
                        options(nostack)
                    );
                }
                value
            }
            #[inline(always)]
            fn write(value: usize) {
                unsafe {
                    asm!(
                        concat!("csrw ", $csr_name, ", {}"),
                        in(reg) value,
                        options(nostack)
                    );
                }
            }
        }
    };
}

DEFINE_SYSREG!(Sstatus, "sstatus");
DEFINE_SYSREG!(Sedeleg, "sedeleg");
DEFINE_SYSREG!(Sideleg, "sideleg");
DEFINE_SYSREG!(Sie, "sie");
DEFINE_SYSREG!(Stvec, "stvec");
DEFINE_SYSREG!(Scounteren, "scounteren");
DEFINE_SYSREG!(Sscratch, "sscratch");
DEFINE_SYSREG!(Sepc, "sepc");
DEFINE_SYSREG!(Scause, "scause");
DEFINE_SYSREG!(Stval, "stval");
DEFINE_SYSREG!(Sip, "sip");

impl Sstatus {
    const SIE: usize = 1 << 1;
    const SPIE: usize = 1 << 5;
    const SPP: usize = 1 << 8;
    pub fn clear_sie() {
        let mut value = Self::read();
        value &= !Self::SIE;
        Self::write(value);
    }
}
