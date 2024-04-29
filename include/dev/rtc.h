#ifndef __RTC_H__
#define __RTC_H__
#include <types.h>

#define GOLDFISH_RTC_BASE 0x101000UL
#define GOLDFISH_RTC_SIZE 0x1000UL

#define GOLDFISH_RTC_TIME_LOW 0x0
#define GOLDFISH_RTC_TIME_HIGH 0x04
#define GOLDFISH_RTC_ALARM_LOW 0x08
#define GOLDFISH_RTC_ALARM_HIGH 0X0c
#define GOLDFISH_RTC_IRQ_ENABLED 0x10
#define GOLDFISH_RTC_ALARM_STATUS 0x18

void rtc_init(size_t base, size_t size);
u64 rtc_read_time(void);
void rtc_write_time(u64 time);
void rtc_turn_um(void);
void rtc_turn_mm(void);
u64 rtc_read_alarm(void);
u32 rtc_alarm_status(void);
u32 rtc_irq_is_enabled(void);
#endif // __RTC_H__