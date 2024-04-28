#include <sbicall.h>

struct sbiret sbi_hart_start(size_t hartid, size_t start_addr, size_t opaque) {
  struct sbiret ret;
  asm volatile("mv a0, %1\n"
               "mv a1, %2\n"
               "mv a2, %3\n"
               "li a6, %4\n"
               "li a7, %5\n"
               "ecall\n"
               "mv %0, a0"
               : "=r"(ret.error)
               : "r"(hartid), "r"(start_addr), "r"(opaque),
                 "i"(SBI_EXT_HSM_HART_START), "i"(SBI_EXT_HSM)
               : "a0", "a1", "a2", "a7", "gp");
  return ret;
}

struct sbiret sbi_hart_stop(void) {
  struct sbiret ret;
  asm volatile("li a6, %1\n"
               "li a7, %2\n"
               "ecall\n"
               "mv %0, a0"
               : "=r"(ret.error)
               : "i"(SBI_EXT_HSM_HART_STOP), "i"(SBI_EXT_HSM)
               : "a0", "a7", "gp");
  return ret;
}

struct sbiret sbi_hart_suspend(size_t suspend_type, size_t resume_addr,
                               size_t opaque) {
  struct sbiret ret = {0, 0};
  return ret;
}

struct sbiret sbi_hart_wakeup(size_t hartid) {
  struct sbiret ret;
  asm volatile("mv a0, %1\n"
               "li a6, %2\n"
               "li a7, %3\n"
               "ecall\n"
               "mv %0, a0"
               : "=r"(ret.error)
               : "r"(hartid), "i"(SBI_EXT_HSM_HART_SUSPEND), "i"(SBI_EXT_HSM)
               : "a0", "a7", "gp");
  return ret;
}

struct sbiret inline sbi_set_timer(size_t stime_value) {
  struct sbiret ret = {0, 0};
  return ret;
}

struct sbiret sbi_hart_get_status(size_t hartid) {
  struct sbiret ret;
  asm volatile("mv a0, %1\n"
               "li a6, %2\n"
               "li a7, %3\n"
               "ecall\n"
               "mv %0, a0"
               : "=r"(ret.error)
               : "r"(hartid), "i"(SBI_EXT_HSM_HART_GET_STATUS), "i"(SBI_EXT_HSM)
               : "a0", "a7", "gp");
  return ret;
}

void sbi_shutdown(void) {
  asm volatile("li a7, %0\n"
               "ecall\n"
               :
               : "i"(SBI_SHUTDOWN));
}