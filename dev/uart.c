#include <dev/uart.h>
#include <types.h>
#include <mm/mm.h>

struct uart_operations_struct {
    void (*uart_putchar)(u32 c);
    u32 (*uart_getchar)(void);
    void (*uart_init)(void);
    struct uart* uart;
};

struct uart {
    struct uart_operations_struct* ops;
    size_t pm_base; 
    size_t pm_size;
};


static u32 ns16550a_uart_um_getchar(void);
static void ns16550a_uart_um_putchar(u32 c);
static u32 ns16550a_uart_mm_getchar(void);
static void ns16550a_uart_mm_putchar(u32 c);



static struct uart_operations_struct ns16550a_uart_mm_ops = {
    .uart_putchar = ns16550a_uart_mm_putchar,
    .uart_getchar = ns16550a_uart_mm_getchar,
    .uart_init = NULL,
    .uart = NULL
};


static struct uart_operations_struct ns16550a_uart_um_ops = {
    .uart_putchar = ns16550a_uart_um_putchar,
    .uart_getchar = ns16550a_uart_um_getchar,
    .uart_init = NULL,
    .uart = NULL
};


static struct uart ns16550a_uart = {
    .ops = NULL,
    .pm_base = 0,
    .pm_size = 0,
};


static u32 ns16550a_uart_um_getchar(void) {
    volatile u8 *uart_base = (u8 *) ns16550a_uart.pm_base;
    while (!(uart_base[5] & 0x01));
    return uart_base[0];
}


static void ns16550a_uart_um_putchar(u32 c) {
    volatile u8 *uart_base = (u8 *) ns16550a_uart.pm_base;
    while (!(uart_base[5] & 0x20));
    uart_base[0] = c;
}


static u32 ns16550a_uart_mm_getchar(void) {
    volatile u8 *uart_base = (u8 *) (ns16550a_uart.pm_base | VIRTUAL_KERNEL_BASE);
    while (!(uart_base[5] & 0x01));
    return uart_base[0];
}

static void ns16550a_uart_mm_putchar(u32 c) {
    volatile u8 *uart_base = (u8 *) (ns16550a_uart.pm_base | VIRTUAL_KERNEL_BASE);
    while (!(uart_base[5] & 0x20));
    uart_base[0] = c;
}


void uart_mm_mapped(void) {
    ns16550a_uart.ops = &ns16550a_uart_mm_ops;
}

void uart_mm_unmapped(void) {
    ns16550a_uart.ops = &ns16550a_uart_um_ops;
}



void uart_putchar(u32 c) {
    ns16550a_uart.ops->uart_putchar(c);
}

u32 uart_getchar() {
    return ns16550a_uart.ops->uart_getchar();
}


void uart_init(void) {
    ns16550a_uart.ops = &ns16550a_uart_um_ops;
    ns16550a_uart_mm_ops.uart = &ns16550a_uart;
    ns16550a_uart_um_ops.uart = &ns16550a_uart;
    ns16550a_uart.pm_base = NS16550A_UART_BASE;
    ns16550a_uart.pm_size = PAGE_SIZE;
    if (ns16550a_uart.ops->uart_init != NULL) {
        ns16550a_uart.ops->uart_init();
    }
}