pub struct TimeVal {
    tv_sec: u64,
    tv_usec: u64,
}

pub struct TimeSpec {
    tv_sec: u64,
    tv_nsec: u64,
}

pub struct TimeZone {
    tz_minuteswest: i32,
    tz_dsttime: i32,
}

const DAYS_PER_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[inline(always)]
fn is_leap_year(year: u32) -> bool {
    return (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
}

fn days_since_epoch(year: u32, month: u32, day: u32) -> u32 {
    let mut days = 0;
    for y in 1970..year {
        days += 365;
        if is_leap_year(y) {
            days += 1;
        }
    }
    for m in 0..month {
        days += DAYS_PER_MONTH[m as usize];
        if m == 1 && is_leap_year(year) {
            days += 1;
        }
    }
    days += day;
    return days;
}

pub struct Tms {
    tms_utime: u64,
    tms_stime: u64,
    tms_cutime: u64,
    tms_cstime: u64,
}
