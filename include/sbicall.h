#ifndef __SBICALL_H_
#define __SBICALL_H_

#include <types.h>

// v0.1 legacy
#define SBI_SET_TIMER 0
#define SBI_CONSOLE_PUTCHAR 1
#define SBI_CONSOLE_GETCHAR 2
#define SBI_CLEAR_IPI 3
#define SBI_SEND_IPI 4
#define SBI_REMOTE_FENCE_I 5
#define SBI_REMOTE_SFENCE_VMA 6
#define SBI_REMOTE_SFENCE_VMA_ASID 7
#define SBI_SHUTDOWN 8

// v0.2 and higher
#define SBI_EXT_BASE 0x10
#define SBI_EXT_TIME 0x54494D45
#define SBI_EXT_IPI 0x735049
#define SBI_EXT_RFENCE 0x52464E43
#define SBI_EXT_HSM 0x48534D
#define SBI_EXT_SRST 0x53525354
#define SBI_EXT_PMU 0x504D55
#define SBI_EXT_DBCN 0x4442434E
#define SBI_EXT_SUSP 0x53555350
#define SBI_EXT_CPPC 0x43505043
#define SBI_EXT_DBTR 0x44425452
#define SBI_EXT_SSE 0x535345

/* SBI return error codes */
#define SBI_SUCCESS 0
#define SBI_ERR_FAILED -1
#define SBI_ERR_NOT_SUPPORTED -2
#define SBI_ERR_INVALID_PARAM -3
#define SBI_ERR_DENIED -4
#define SBI_ERR_INVALID_ADDRESS -5
#define SBI_ERR_ALREADY_AVAILABLE -6
#define SBI_ERR_ALREADY_STARTED -7
#define SBI_ERR_ALREADY_STOPPED -8
#define SBI_ERR_NO_SHMEM -9
#define SBI_ERR_INVALID_STATE -10
#define SBI_ERR_BAD_RANGE -11

/* SBI function IDs for HSM extension */
#define SBI_EXT_HSM_HART_START 0x0
#define SBI_EXT_HSM_HART_STOP 0x1
#define SBI_EXT_HSM_HART_GET_STATUS 0x2
#define SBI_EXT_HSM_HART_SUSPEND 0x3

struct sbiret {
  size_t error;
  size_t value;
};

struct sbiret sbi_call(size_t, size_t, size_t, size_t);

// legacy
struct sbiret sbi_set_timer(size_t stime_value);

struct sbiret sbi_hart_start(size_t hartid, size_t start_addr, size_t opaque);

struct sbiret sbi_hart_stop(void);

struct sbiret sbi_hart_get_status(size_t hartid);

struct sbiret sbi_hart_suspend(size_t suspend_type, size_t resume_addr,
                               size_t opaque);

struct sbiret sbi_hart_wakeup(size_t hartid);

void sbi_shutdown(void);

#endif
