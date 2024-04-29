#include <dev/rtc.h>
#include <mm/mm.h>
static void _rtc_init(size_t base, size_t size);
static u64 rtc_read_time_mm(void);
static void rtc_write_time_mm(u64 time);
static void rtc_write_time_um(u64 time);
static u64 rtc_read_time_um(void);
static u32 rtc_irq_is_enabled_um(void);
static u32 rtc_irq_is_enabled_mm(void);
static u32 rtc_alarm_status_um(void);
static u32 rtc_alarm_status_mm(void);
static u64 rtc_read_alarm_um(void);

struct rtc_operations_struct {
  void (*init)(size_t base, size_t size);
  u64 (*read_time)(void);
  void (*write_time)(u64 time);
  u64 (*read_alarm)(void);
  u32 (*alarm_status)(void);
  u32 (*irq_is_enabled)(void);

  struct rtc *rtc;
};

struct rtc {
  size_t base;
  size_t size;
  struct rtc_operations_struct *ops;
};

static struct rtc_operations_struct rtc_um_ops = {
    .init = _rtc_init,
    .read_time = rtc_read_time_um,
    .write_time = rtc_write_time_um,
    .alarm_status = rtc_alarm_status_um,
    .irq_is_enabled = rtc_irq_is_enabled_um,
    .read_alarm = rtc_read_alarm_um,
    .rtc = NULL,
};

static struct rtc_operations_struct rtc_mm_ops = {
    .init = _rtc_init,
    .read_time = rtc_read_time_mm,
    .write_time = rtc_write_time_mm,
    .alarm_status = rtc_alarm_status_mm,
    .irq_is_enabled = rtc_irq_is_enabled_mm,
    .read_alarm = rtc_read_alarm_um,
    .rtc = NULL,
};

static struct rtc goldfish_rtc = {
    .base = 0,
    .size = 0,
    .ops = &rtc_um_ops,
};

static u32 rtc_irq_is_enabled_um(void) {
  return *(volatile u32 *)(goldfish_rtc.base + GOLDFISH_RTC_IRQ_ENABLED);
}

static u32 rtc_irq_is_enabled_mm(void) {
  return *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                           GOLDFISH_RTC_IRQ_ENABLED);
}

static u32 rtc_alarm_status_um(void) {
  return *(volatile u32 *)(goldfish_rtc.base + GOLDFISH_RTC_ALARM_STATUS);
}

static u32 rtc_alarm_status_mm(void) {
  return *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                           GOLDFISH_RTC_ALARM_STATUS);
}

static u64 rtc_read_alarm_um(void) {
  u32 low = *(volatile u32 *)(goldfish_rtc.base + GOLDFISH_RTC_ALARM_LOW);
  u64 high = *(volatile u32 *)(goldfish_rtc.base + GOLDFISH_RTC_ALARM_HIGH);
  return (high << 32) | low;
}

static u64 rtc_read_alarm_mm(void) {
  u32 low = *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                              GOLDFISH_RTC_ALARM_LOW);
  u64 high = *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                               GOLDFISH_RTC_ALARM_HIGH);
  return (high << 32) | low;
}

static u64 rtc_read_time_mm(void) {
  u32 low = *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                              GOLDFISH_RTC_TIME_LOW);
  u64 high = *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                               GOLDFISH_RTC_TIME_HIGH);
  return (high << 32) | low;
}

static void rtc_write_time_mm(u64 time) {
  *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                    GOLDFISH_RTC_TIME_HIGH) = time >> 32;
  *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                    GOLDFISH_RTC_TIME_LOW) = time & 0xffffffff;
}

static void rtc_write_time_um(u64 time) {
  *(volatile u32 *)(goldfish_rtc.base + GOLDFISH_RTC_TIME_HIGH) = time >> 32;
  *(volatile u32 *)(goldfish_rtc.base + GOLDFISH_RTC_TIME_LOW) =
      time & 0xffffffff;

  *(volatile u64 *)(goldfish_rtc.base + GOLDFISH_RTC_TIME_LOW) = time;
}

static u64 rtc_read_time_um(void) {
  u32 low = *(volatile u32 *)(goldfish_rtc.base + GOLDFISH_RTC_TIME_LOW);
  u64 high = *(volatile u32 *)(goldfish_rtc.base + GOLDFISH_RTC_TIME_HIGH);
  return (high << 32) | low;
}

static void _rtc_init(size_t base, size_t size) {
  rtc_mm_ops.rtc = &goldfish_rtc;
  rtc_um_ops.rtc = &goldfish_rtc;
  goldfish_rtc.base = base;
  goldfish_rtc.size = size;
  goldfish_rtc.ops = &rtc_um_ops;
}

void rtc_init(size_t base, size_t size) { goldfish_rtc.ops->init(base, size); }

void rtc_turn_um(void) { goldfish_rtc.ops = &rtc_um_ops; }

void rtc_turn_mm(void) { goldfish_rtc.ops = &rtc_mm_ops; }

u64 rtc_read_time(void) { return goldfish_rtc.ops->read_time(); }
void rtc_write_time(u64 time) { goldfish_rtc.ops->write_time(time); }
u64 rtc_read_alarm(void) { return goldfish_rtc.ops->read_alarm(); }
u32 rtc_alarm_status(void) { return goldfish_rtc.ops->alarm_status(); }
u32 rtc_irq_is_enabled(void) { return goldfish_rtc.ops->irq_is_enabled(); }