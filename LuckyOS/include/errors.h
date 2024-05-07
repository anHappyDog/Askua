#ifndef __ERROR_H_
#define __ERROR_H_
#include <types.h>

typedef uint32_t error_t;

#define E_OK 0
#define E_NO_MEM -1
#define E_NO_FREE_PAGE -2
#define E_INVAL -3

#endif // __ERROR_H_