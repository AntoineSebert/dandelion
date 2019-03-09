/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	17/02/2019
 */

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]
#![feature(asm)]

use core::panic::PanicInfo;
use dandelion::{exit_qemu, serial_println};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	use x86_64::software_interrupt;
	use dandelion::interrupts::init_idt;

	init_idt();
	unsafe { software_interrupt!(50); }

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
