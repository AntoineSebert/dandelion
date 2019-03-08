/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	27/01/2019
 */

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use dandelion::{exit_qemu, serial_println};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	serial_println!("ok");

	unsafe {
		exit_qemu();
	}
	loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	serial_println!("failed");
	serial_println!("{}", info);

	unsafe {
		exit_qemu();
	}
	loop {}
}
