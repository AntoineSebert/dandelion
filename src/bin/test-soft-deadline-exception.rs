/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	17/02/2019
 */

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

extern crate dandelion;
extern crate x86_64;

use core::panic::PanicInfo;
use dandelion::{exit_qemu, serial_println};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	use dandelion::interrupts::{interrupt_indexes::RealTime, int};

	int(RealTime::SoftDeadline);
	serial_println!("ok");

	unsafe {
		exit_qemu();
	}
	loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	serial_println!("failed");
	serial_println!("{}", _info);

	unsafe {
		exit_qemu();
	}
	loop {}
}
