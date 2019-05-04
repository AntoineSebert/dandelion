// https://wiki.osdev.org/HPET
// https://wiki.osdev.org/APIC_timer
// https://wiki.osdev.org/Time_And_Date

use cmos::RTCDateTime;
use core::time::Duration;

/// Return the current datetime as `RTCDateTime`.
pub fn get_datetime() -> RTCDateTime {
	use cmos::{CMOSCenturyHandler, CMOS};

	let mut cmos = unsafe { CMOS::new() };
	cmos.read_rtc(CMOSCenturyHandler::CurrentYear(2019))
}

/// Return the difference between two `RTCDateTime` as a `Duration`.
pub fn get_duration(first: RTCDateTime, second: RTCDateTime) -> Duration {
	to_duration(dt_sub_dt(first, second))
}

// Operations

// implement derive traits in cmos crate

/// If first < second, the fields in the returned RTCDateTime equal to 0.
pub fn dt_sub_dt(first: RTCDateTime, second: RTCDateTime) -> RTCDateTime {
	RTCDateTime {
		second: first.second - second.second,
		minute: first.minute - second.minute,
		hour: first.hour - second.hour,
		day: first.day - second.day,
		month: first.month - second.month,
		year: first.year - second.year,
	}
}

/// Assumes that both `RTCDateTime` parameters are valid.
/// Returns `Some(RTCDateTime)` if the addition could be performed, or `None` if the result is greater than the maximal `RTCDateTime` possible.
pub fn dt_add_dt(first: RTCDateTime, second: RTCDateTime) -> Option<RTCDateTime> {
	use core::usize::MAX;

	if (MAX - first.year) < second.year {
		return None;
	}

	let mut result = RTCDateTime {
		second: first.second + second.second,
		minute: first.minute + second.minute,
		hour: first.hour + second.hour,
		day: first.day + second.day,
		month: first.month + second.month,
		year: 0,
	};

	let remove_overflow = | a: &mut u8, b: &mut u8, limit: u8 | {
		if limit < *a {
			*a -= limit;
			*b += 1;
		}
	};

	remove_overflow(&mut result.second, &mut result.minute, 60);
	remove_overflow(&mut result.minute, &mut result.hour, 60);
	remove_overflow(&mut result.hour, &mut result.day, 24);
	remove_overflow(&mut result.day, &mut result.month, 31);

	result.year = first.year + second.year;
	if result.year == MAX && 12 < result.month {
		None
	} else {
		result.month -= 12;
		result.year += 1;
		Some(result)
	}
}

/// Adds a `Duration` to an `RTCDateTime`.
/// Returns `Some(RTCDateTime)` if the operation could be performed, or `None` if the result is greater than the maximal `RTCDateTime` possible.
pub fn dt_add_du(datetime: RTCDateTime, duration: Duration) -> Option<RTCDateTime> {
	dt_add_dt(datetime, to_rtcdatetime(duration))
}

/// Converts a `RTCDateTime` into a `Duration` without checking.
/// However it is possible that `u64::MAX` seconds is greater than the maximal `RTCDateTime` possible.
pub fn to_duration(datetime: RTCDateTime) -> Duration {
	Duration::from_secs(
		datetime.second as u64
		+ (datetime.minute * 60) as u64
		+ (datetime.hour as u64 * 3_600)
		+ (datetime.day as u64 * 86_400)
		+ (datetime.month as u64 * 2_628_000)
		+ (datetime.year * 31_536_000) as u64
	)
}

/// Converts a `Duration` into a `RTCDateTime` without ckecking.
pub fn to_rtcdatetime(duration: Duration) -> RTCDateTime {
	let apply_divisor = |time: &mut u64, divisor: u64| -> u8 {
		let left = *time / divisor;
		*time %= divisor;
		left as u8
	};
	let mut time: u64 = duration.as_secs();
	let mut result = RTCDateTime { second: 0, minute: 0, hour: 0, day: 0, month: 0, year: 0 };

	result.year = apply_divisor(&mut time, 31_536_000) as usize;
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
pub fn intersect(a: (RTCDateTime, RTCDateTime), b: (RTCDateTime, RTCDateTime)) -> bool { (a.0 < b.0 && a.1 <= b.0) || (b.1 <= a.0 && b.1 < a.1) }
