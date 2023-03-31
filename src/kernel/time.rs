// https://wiki.osdev.org/HPET
// https://wiki.osdev.org/APIC_timer
// https://wiki.osdev.org/Time_And_Date

use cmos_rtc::Time;
use core::time::Duration;

/// Return the current datetime as `Time`.
#[must_use]
pub fn get_datetime() -> Time {
	let mut cmos = cmos_rtc::ReadRTC::new(0x00, 0x00);

	cmos.read()
}

// Operations

/// Return the difference between two `Time` as a `Duration`.
#[must_use]
pub fn get_duration(first: Time, second: Time) -> Duration { to_duration(first) - to_duration(second) }

/// If first < second, the fields in the returned Time equal to 0.
#[must_use]
pub fn dt_sub_dt(first: Time, second: Time) -> Time {
	Time {
		second: first.second - second.second,
		minute: first.minute - second.minute,
		hour: first.hour - second.hour,
		day: first.day - second.day,
		month: first.month - second.month,
		year: first.year - second.year,
		century: first.century - second.century,
	}
}

/// Assumes that both `Time` parameters are valid.
/// Returns `Some(Time)` if the addition could be performed.
/// Returns `None` if the result is greater than `Time::max()`.
#[must_use]
pub fn dt_add_dt(first: Time, second: Time) -> Option<Time> {
	use core::usize::MAX;

	if (MAX - first.year as usize) < second.year.into() {
		None
	} else {
		let remove_overflow = |a: &mut u8, b: &mut u8, limit: u8| {
			if limit < *a {
				*a -= limit;
				*b += 1;
			}
		};

		let mut result = Time {
			second: first.second + second.second,
			minute: first.minute + second.minute,
			hour: first.hour + second.hour,
			day: first.day + second.day,
			month: first.month + second.month,
			year: first.year + second.year,
			century: first.century + second.century,
		};

		remove_overflow(&mut result.second, &mut result.minute, 60);
		remove_overflow(&mut result.minute, &mut result.hour, 60);
		remove_overflow(&mut result.hour, &mut result.day, 24);
		remove_overflow(&mut result.day, &mut result.month, 31);

		if usize::from(result.year) == MAX && 12 < result.month {
			None
		} else {
			result.month -= 12;
			result.year += 1;
			Some(result)
		}
	}
}

/// Adds a `Duration` to an `Time`.
/// Returns `Some(Time)` if the operation could be performed.
/// Returns `None` if the result is greater than `Time::max()`.
#[must_use]
pub fn dt_add_du(datetime: Time, duration: Duration) -> Option<Time> { dt_add_dt(datetime, to_rtcdatetime(duration)) }

/// Converts a `Time` into a `Duration` without checking.
// add checks
#[must_use]
pub fn to_duration(datetime: Time) -> Duration {
	Duration::from_secs(
		u64::from(datetime.second)
			+ u64::from(datetime.minute) * 60
			+ u64::from(datetime.hour) * 3_600
			+ u64::from(datetime.day) * 86_400
			+ u64::from(datetime.month) * 2_628_000
			+ (datetime.year as u64 * 31_536_000),
	)
}

/// Converts a `Duration` into a `Time` without ckecking.
#[must_use]
pub fn to_rtcdatetime(duration: Duration) -> Time {
	let apply_divisor = |time: &mut u64, divisor: u64| -> u8 {
		let left = *time / divisor;
		*time %= divisor;
		left as u8
	};
	let mut time: u64 = duration.as_secs();
	let mut result = Time::default();

	result.year = apply_divisor(&mut time, 31_536_000);
	result.month = apply_divisor(&mut time, 2_628_000);
	result.day = apply_divisor(&mut time, 86_400);
	result.hour = apply_divisor(&mut time, 3_600);
	result.minute = apply_divisor(&mut time, 60);
	result.second = time as u8;

	result
}

/// Returns `true` if there is an intersection between the timespan parameters, and `false` otherwise.
/// An inclusion is considered as an intersection.
/// Assumes that the timespans are ordered.
#[must_use]
pub fn intersect(a: (Time, Time), b: (Time, Time)) -> bool { (a.0 < b.0 && a.1 <= b.0) || (b.1 <= a.0 && b.1 < a.1) }
