/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

use cmos::RTCDateTime;
use core::time::Duration;

pub fn get_datetime() -> RTCDateTime {
	use crate::kernel::CMOS;
	use cmos::CMOSCenturyHandler;

	CMOS.lock().read_rtc(CMOSCenturyHandler::CurrentYear(2019))
}

pub fn get_duration(first: RTCDateTime, second:RTCDateTime) -> Duration {
	Duration::new(0, 0)
}