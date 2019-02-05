/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/02/2019
 */

#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate dandelion;

use core::panic::PanicInfo;
use dandelion::{exit_qemu, serial_println};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	dandelion::interrupts::init_idt();

	x86_64::instructions::int3();

	serial_println!("ok");

	unsafe {
		exit_qemu();
	}
	loop {}
}

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
