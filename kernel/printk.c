#include <dev/uart.h>
#include <lock/spinlock.h>
#include <printk.h>
#include <sbicall.h>

spinlock_t printk_lock = SPIN_INIT;

static void outputk(void *data, const char *buf, size_t len) {
  for (int i = 0; i < len; i++) {
    uart_putchar(buf[i]);
  }
}

void printk(const char *fmt, ...) {
  spin_lock(&printk_lock);

  va_list ap;
  va_start(ap, fmt);
  vprintfmt(outputk, NULL, fmt, ap);
  va_end(ap);

  spin_unlock(&printk_lock);
}

void print_tf(const trapframe_t *tf) {
  printk("trapframe at " HEX_BITS "\n"
         "  ra: " HEX_BITS "\n"
         "  gp: " HEX_BITS "\n"
         "  tp: " HEX_BITS "\n"
         "  a0: " HEX_BITS "\n"
         "  a1: " HEX_BITS "\n"
         "  a2: " HEX_BITS "\n"
         "  a3: " HEX_BITS "\n"
         "  a4: " HEX_BITS "\n"
         "  a5: " HEX_BITS "\n"
         "  a6: " HEX_BITS "\n"
         "  a7: " HEX_BITS "\n"
         " sepc:   " HEX_BITS ", 	sstatus: " HEX_BITS "\n"
         " scause: " HEX_BITS ", 	stval: " HEX_BITS "\n"
         " sscratch: " HEX_BITS "\n",
         tf, tf->regs[1], tf->regs[3], tf->regs[4], tf->regs[10], tf->regs[11],
         tf->regs[12], tf->regs[13], tf->regs[14], tf->regs[15], tf->regs[16],
         tf->regs[17], tf->sepc, tf->sstatus, tf->scause, tf->stval,
         tf->sscratch);
}

void _panic(const char *file, int line, const char *func, const char *fmt,
            ...) {
  printk("panic at %s:%d (%s): ", file, line, func);

  va_list ap;
  va_start(ap, fmt);
  vprintfmt(outputk, NULL, fmt, ap);
  va_end(ap);
  sbi_shutdown();
  while (1)
    ;
}
