#ifndef __PRINTK_H_
#define __PRINTK_H_
#include <stdarg.h>
#include <lib/print.h>
#include <types.h>
#include <trapframe.h>

void printk(const char *fmt, ...);
void print_tf(const trapframe_t *tf);
void _panic(const char *, int, const char *, const char *, ...)__attribute__((noreturn));
// void print_tf(const trapframe_t* tf);

#define HEX_BITS "%016lx"

#define panic(...) _panic(__FILE__, __LINE__, __func__, __VA_ARGS__)

#define panic_on(expr,msg)                                                                             \
	do {                                                                                       \
		int _r = (expr);                                                                   \
		if (_r != 0) {                                                                     \
			panic( #msg "\n['" #expr "]' returned %d", _r);                                      \
		}                                                                                  \
	} while (0)


#endif