/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

use cmos::RTCDateTime;

pub fn get_datetime() -> RTCDateTime {
	use crate::kernel::CMOS;
	use cmos::CMOSCenturyHandler;

	CMOS.lock().read_rtc(CMOSCenturyHandler::CurrentYear(2019))
}
