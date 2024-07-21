#[cfg(target_arch = "riscv64")]
// x0: Zero register (固定值0，任何对x0的写操作都会被忽略)
// x1: ra (返回地址寄存器)
// x2: sp (栈指针寄存器)
// x3: gp (全局指针寄存器)
// x4: tp (线程指针寄存器)
// x5 - x7: t0 - t2 (临时/可用于调用者保存的寄存器)
// x8: s0/fp (保存寄存器或帧指针)
// x9: s1 (保存寄存器)
// x10 - x11: a0 - a1 (函数参数/返回值)
// x12 - x17: a2 - a7 (函数参数)
// x18 - x27: s2 - s11 (被调用者保存寄存器)
// x28 - x31: t3 - t6 (临时/可用于调用者保存的寄存器)
pub struct KernelContext {
    regs: [usize; 31],
}

// u64 kernel_satp;  // 保存内核页表
// u64 trap_handler; // 内核态异常针对用户异常的处理函数（C函数）
// u64 epc;	  // 用户的epc
// u64 hartid;	  // 当前的hartid，取自tp寄存器
pub struct UserContext {
    regs: [usize; 31],
    ksp: usize,
    kstap: usize,
    trap_handler: usize,
    epc: usize,
    hartid: usize,
}
