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
