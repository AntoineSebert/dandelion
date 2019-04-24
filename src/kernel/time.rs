/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

// https://wiki.osdev.org/HPET
// https://wiki.osdev.org/APIC_timer
// https://wiki.osdev.org/Time_And_Date

// https://doc.rust-lang.org/book/operators-and-overloading.html
// https://doc.rust-lang.org/core/ops/index.html

use cmos::RTCDateTime;
use core::time::Duration;

pub fn get_datetime() -> RTCDateTime {
	use cmos::{CMOSCenturyHandler, CMOS};

	let mut cmos = unsafe { CMOS::new() };
	cmos.read_rtc(CMOSCenturyHandler::CurrentYear(2019))
}

pub fn get_duration(first: RTCDateTime, second: RTCDateTime) -> Duration {
	let _result: RTCDateTime = RTCDateTime {
		second: first.second - second.second,
		minute: first.minute - second.minute,
		hour: first.hour - second.hour,
		day: first.day - second.day,
		month: first.month - second.month,
		year: first.year - second.year,
	};

	Duration::new(0, 0)
}

/// Operations

/// if first < second, the fields in the returned RTCDateTime equal to 0.
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

pub fn dt_add_dt(first: RTCDateTime, second: RTCDateTime) -> RTCDateTime {
	RTCDateTime {
		second: first.second + second.second,
		minute: first.minute + second.minute,
		hour: first.hour + second.hour,
		day: first.day + second.day,
		month: first.month + second.month,
		year: first.year + second.year,
	}
}

pub fn dt_add_du(datetime: RTCDateTime, duration: Duration) -> RTCDateTime {
	dt_add_dt(datetime, to_rtcdatetime(duration))
}

pub fn to_duration(datetime: RTCDateTime) -> Duration {
	let mut result = Duration::new(datetime.second.into(), 0);

	result += Duration::from_secs((datetime.minute * 60).into());
	result += Duration::from_secs((datetime.hour as u64 * 3_600).into());
	result += Duration::from_secs((datetime.day as u64 * 86_400).into());
	result += Duration::from_secs((datetime.month as u64 * 2_628_000).into());
	result += Duration::from_secs(datetime.year as u64 * 31_536_000);

	result
}

pub fn to_rtcdatetime(duration: Duration) -> RTCDateTime {
	// change &u64 to &Duration
	let apply_divisor = |time: &mut u64, divisor: u64| -> u8 {
		let left = *time / divisor;
		*time %= divisor;
		left as u8
	};
	let mut time: u64 = duration.as_secs();
	let mut result = RTCDateTime {
		second: 0,
		minute: 0,
		hour: 0,
		day: 0,
		month: 0,
		year: 0,
	};
	result.year = apply_divisor(&mut time, 31_536_000) as usize;
	result.month = apply_divisor(&mut time, 2_628_000);
	result.day = apply_divisor(&mut time, 86_400);
	result.hour = apply_divisor(&mut time, 3_600);
	result.minute = apply_divisor(&mut time, 60);
	result.second = time as u8;

	result
}

pub fn intersect(a: (RTCDateTime, RTCDateTime), b: (RTCDateTime, RTCDateTime)) -> bool {
	(a.0 < b.0 && a.1 <= b.0) || (b.1 <= a.0 && b.1 < a.1)
}
pub type Datetimespan = (RTCDateTime, RTCDateTime);

pub type TimeboundConstraint = (Datetimespan, Datetimespan, Duration);// (min, max), (current start, current end), duration