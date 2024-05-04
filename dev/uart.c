#include <dev/uart.h>
#include <mm/mm.h>
#include <types.h>

struct uart_operations_struct {
  void (*uart_putchar)(u32 c);
  u32 (*uart_getchar)(void);
  void (*uart_init)(void);
  struct uart *uart;
};

struct uart {
  struct uart_operations_struct *ops;
  size_t pm_base;
  size_t pm_size;
};

static u32 ns16550a_uart_getchar(void);
static void ns16550a_uart_putchar(u32 c);

static struct uart_operations_struct ns16550a_uart_ops = {
    .uart_putchar = ns16550a_uart_putchar,
    .uart_getchar = ns16550a_uart_getchar,
    .uart_init = NULL,
    .uart = NULL};

static struct uart ns16550a_uart = {
    .ops = &ns16550a_uart_ops,
    .pm_base = NS16550A_UART_BASE,
    .pm_size = PAGE_SIZE,
};

static u32 ns16550a_uart_getchar(void) {
  volatile u8 *uart_base = (u8 *)(ns16550a_uart.pm_base | VIRTUAL_KERNEL_BASE);
  while (!(uart_base[5] & 0x01))
    ;
  return uart_base[0];
}

static void ns16550a_uart_putchar(u32 c) {
  volatile u8 *uart_base = (u8 *)(ns16550a_uart.pm_base | VIRTUAL_KERNEL_BASE);
  uart_base[0] = c;
}

void uart_putchar(u32 c) { ns16550a_uart.ops->uart_putchar(c); }

u32 uart_getchar() { return ns16550a_uart.ops->uart_getchar(); }
