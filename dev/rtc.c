#include <dev/rtc.h>
#include <mm/mm.h>
static void _rtc_init(size_t base, size_t size);
static u64 _rtc_read_time(void);
static void _rtc_write_time(u64 time);
static u32 _rtc_irq_is_enabled(void);
static u32 _rtc_alarm_status(void);
static u64 _rtc_read_alarm(void);


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


static struct rtc_operations_struct rtc_ops = {
    .init = _rtc_init,
    .read_time = _rtc_read_time,
    .write_time = _rtc_write_time,
    .alarm_status = _rtc_alarm_status,
    .irq_is_enabled = _rtc_irq_is_enabled,
    .read_alarm = _rtc_read_alarm,
    .rtc = NULL,
};

static struct rtc goldfish_rtc = {
    .base = GOLDFISH_RTC_BASE,
    .size = GOLDFISH_RTC_SIZE,
    .ops = &rtc_ops,
};



static u32 _rtc_irq_is_enabled(void) {
  return *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                           GOLDFISH_RTC_IRQ_ENABLED);
}



static u32 _rtc_alarm_status(void) {
  return *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                           GOLDFISH_RTC_ALARM_STATUS);
}



static u64 _rtc_read_alarm(void) {
  u32 low = *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                              GOLDFISH_RTC_ALARM_LOW);
  u64 high = *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                               GOLDFISH_RTC_ALARM_HIGH);
  return (high << 32) | low;
}

static u64 _rtc_read_time(void) {
  u32 low = *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                              GOLDFISH_RTC_TIME_LOW);
  u64 high = *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                               GOLDFISH_RTC_TIME_HIGH);
  return (high << 32) | low;
}

static void _rtc_write_time(u64 time) {
  *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                    GOLDFISH_RTC_TIME_HIGH) = time >> 32;
  *(volatile u32 *)(goldfish_rtc.base + VIRTUAL_KERNEL_BASE +
                    GOLDFISH_RTC_TIME_LOW) = time & 0xffffffff;
}

static void _rtc_init(size_t base, size_t size) {
  rtc_ops.rtc = &goldfish_rtc;
  goldfish_rtc.base = base;
  goldfish_rtc.size = size;
  goldfish_rtc.ops = &rtc_ops;
}

void rtc_init(size_t base, size_t size) { goldfish_rtc.ops->init(base, size); }

u64 rtc_read_time(void) { return goldfish_rtc.ops->read_time(); }
void rtc_write_time(u64 time) { goldfish_rtc.ops->write_time(time); }
u64 rtc_read_alarm(void) { return goldfish_rtc.ops->read_alarm(); }
u32 rtc_alarm_status(void) { return goldfish_rtc.ops->alarm_status(); }
u32 rtc_irq_is_enabled(void) { return goldfish_rtc.ops->irq_is_enabled(); }