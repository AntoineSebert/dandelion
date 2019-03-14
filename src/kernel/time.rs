/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

use crate::kernel::CMOS;
use cmos::CMOSCenturyHandler;
use x86_64::instructions::{interrupts::without_interrupts, port::Port};

// replace by function in cmos crate
pub fn set_register(register: u8, value: u8) {
	if register == 0x8A || register == 0x8B || register == 0x8C {
		panic!()
	}

	without_interrupts(|| {
		unsafe {
			Port::<u8>::new(0x70).write(register);
			Port::<u8>::new(0x71).write(value);
		}
	})
}

pub fn get_datetime() -> u64 {
	let rtc = CMOS.lock().read_rtc(CMOSCenturyHandler::CurrentYear(2019));
	/*
		let mut total: u64 = 0;
		for value in output.iter() {
			total += *value as u64;
		}
	*/
	u64::from(rtc.minute)
}
