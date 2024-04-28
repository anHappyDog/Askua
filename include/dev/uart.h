#ifndef __UART_H_
#define __UART_H_
#include <stdarg.h>
#include <types.h>
// Maybe we can get it from the fdt.
#define NS16550A_UART_BASE 0x10000000UL
#define NS16550A_UART_DATA (NS16550A_UART_BASE + 0x0UL)

void uart_mm_mapped(void);
void uart_mm_unmapped(void);
void uart_putchar(u32 c);
u32 uart_getchar();
void uart_init(void);

#endif // __UART_H_