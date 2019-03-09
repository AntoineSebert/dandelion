/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

// https://github.com/noahrinehart/cmos
// https://docs.rs/cmos/0.1.1/cmos/

use cmos::{CMOS, CMOSCenturyHandler};

let mut cmos = unsafe { CMOS::new() };
let rtc = cmos.read_rtc(CMOSCenturyHandler::CurrentYear(2019));
println!("{:?}", rtc);