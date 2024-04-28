#include <asm/csr.h>
#include <dev/plic.h>
#include <printk.h>
#include <sbicall.h>
#include <smp.h>
#include <trap.h>
#include <trapframe.h>
#include <mm/mm.h>

extern void exc_gen_entry(void);
void trap_handler(trapframe_t *tf);

void trap_init(void)
{
	printk("trap_init,entrt is %016lx\n",(size_t)exc_gen_entry | VIRTUAL_KERNEL_BASE);
	
	write_stvec(((size_t)exc_gen_entry | VIRTUAL_KERNEL_BASE) | STVEC_VECTOR);
	write_sie(read_sie() | SIE_SEIE | SIE_STIE | SIE_SSIE);
	write_sstatus(read_sstatus() | SSTATUS_SIE);
}


void enable_irq() {
	write_sstatus(read_sstatus() | SSTATUS_SIE);
}

void disable_irq() {
	write_sstatus(read_sstatus() & ~SSTATUS_SIE);
}

u8 is_irq_enabled() {
	return read_sstatus() & SSTATUS_SIE;
}


__attribute__((noreturn)) void handle_reserved_int()
{
	panic("Reserved interrupt happened!\n");
}

void handle_reserved_trap(trapframe_t *tf)
{
	print_tf(tf);
	panic("Reserved trap happened!\n");
}

void handle_instruction_address_misaligned(trapframe_t *tf)
{
	print_tf(tf);
	panic("Instruction address misaligned!\n");
}

void handle_instruction_access_fault(trapframe_t *tf)
{
	print_tf(tf);
	panic("Instruction access fault!\n");
}

void handle_illegal_instruction(trapframe_t *tf)
{
	print_tf(tf);
	panic("Illegal instruction!\n");
}

void handle_breakpoint(trapframe_t *tf)
{
	print_tf(tf);
	panic("Breakpoint!\n");
}

void handle_load_address_misaligned(trapframe_t *tf)
{
	print_tf(tf);
	panic("Load address misaligned!\n");
}

void handle_load_access_fault(trapframe_t *tf)
{
	print_tf(tf);
	panic("Load access fault!\n");
}

void handle_store_address_misaligned(trapframe_t *tf)
{
	print_tf(tf);
	panic("Store/AMO address misaligned!\n");
}

void handle_store_access_fault(trapframe_t *tf)
{
	print_tf(tf);
	panic("Store/AMO access fault!\n");
}

void handle_ecall_from_umode(trapframe_t *tf)
{
	print_tf(tf);
	panic("Environment call from U-mode!\n");
}

void handle_ecall_from_smode(trapframe_t *tf)
{
	print_tf(tf);
	panic("Environment call from S-mode!\n");
}

void handle_ecall_from_mmode(trapframe_t *tf)
{
	print_tf(tf);
	panic("Environment call from M-mode!\n");
}

void handle_instruction_page_fault(trapframe_t *tf)
{
	print_tf(tf);
	panic("Instruction page fault!\n");
}

void handle_load_page_fault(trapframe_t *tf)
{
	print_tf(tf);
	panic("Load page fault!\n");
}

void handle_store_page_fault(trapframe_t *tf)
{
	print_tf(tf);
	panic("Store/AMO page fault!\n");
}

__attribute__((interrupt("supervisor"))) void handle_s_software_int()
{
	sbi_clear_ipi();
	printk("[%x]S-mode software interrupt!\n", SMP_GET_HARTID());
}

__attribute__((interrupt("machine"))) void handle_m_software_int()
{
	panic("M-mode software interrupt!\n");
}

__attribute__((interrupt("supervisor"))) void handle_s_timer_int()
{
	size_t hartid = SMP_GET_HARTID();
	printk("[%08x] S-mode timer interrupt!\n", hartid);
	sbi_set_timer(0x1000000 + read_time());
}

__attribute__((interrupt("machine"))) void handle_m_timer_int()
{

	panic("M-mode timer interrupt!\n");
}

__attribute__((interrupt("machine"))) void handle_m_extern_int()
{
	panic("M-mode external interrupt!\n");
}

void __attribute__((interrupt("supervisor"))) handle_s_extern_int()
{
	size_t hartid = SMP_GET_HARTID();
	 uint32_t irq = plic_claim(SIFIVE_BASE_ADDR, 2 * hartid + 1);
	//uint32_t irq = 0x8;
	printk("[%08x] S-mode extern[%08X] interrupt!\n", hartid,irq);
	plic_complete(SIFIVE_BASE_ADDR,2 * hartid + 1, irq);
}

void trap_handler(trapframe_t *tf)
{
	size_t cause = read_scause();
	cause = (cause << 2) >> 2;
	switch (cause) {
	case INSTR_ADDR_MISALIGNED: // Instruction address misaligned
		handle_instruction_address_misaligned(tf);
		break;
	case INSTR_ACCESS_FAULT: // Instruction access fault
		handle_instruction_access_fault(tf);
		break;
	case ILLEGAL_INSTRUCTION: // Illegal instruction
		handle_illegal_instruction(tf);
		break;
	case BREAKPOINT: // Breakpoint
		handle_breakpoint(tf);
		break;
	case LOAD_ADDR_MISALIGNED: // Load address misaligned
		handle_load_address_misaligned(tf);
		break;
	case LOAD_ACCESS_FAULT: // Load access fault
		handle_load_access_fault(tf);
		break;
	case STORE_ADDR_MISALIGNED: // Store/AMO address misaligned
		handle_store_address_misaligned(tf);
		break;
	case STORE_ACCESS_FAULT: // Store/AMO access fault
		handle_store_access_fault(tf);
		break;
	case ENV_CALL_FROM_U_MODE: // Environment call from U-mode
		handle_ecall_from_umode(tf);
		break;
	case ENV_CALL_FROM_S_MODE: // Environment call from S-mode
		handle_ecall_from_smode(tf);
		break;
	case ENV_CALL_FROM_M_MODE: // Environment call from M-mode
		handle_ecall_from_mmode(tf);
		break;
	case INSTR_PAGE_FAULT: // Instruction page fault
		handle_instruction_page_fault(tf);
		break;
	case LOAD_PAGE_FAULT: // Load page fault
		handle_load_page_fault(tf);
		break;
	case STORE_PAGE_FAULT: // Store/AMO page fault
		handle_store_page_fault(tf);
		break;
	default:
		panic("Unknown trap cause!\n");
		break;
	}
}
