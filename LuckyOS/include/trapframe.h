#ifndef __TRAPFRAME_H__
#define __TRAPFRAME_H__
#include <types.h>

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
typedef struct trapframe trapframe_t;

struct trapframe {
  size_t regs[32];
  size_t sstatus;
  size_t sepc;
  size_t sscratch;
  size_t stval;
  size_t scause;
};
typedef struct trapframe trapframe_t;

#endif // __TRAPFRAME_H__