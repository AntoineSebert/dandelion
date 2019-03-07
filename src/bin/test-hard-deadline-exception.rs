/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	17/02/2019
 */

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]
#![feature(asm)]

extern crate dandelion;
extern crate x86_64;

use core::panic::PanicInfo;
use dandelion::{exit_qemu, serial_println};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	//use x86_64::instructions::interrupts::software_interrupt;
	use dandelion::interrupts::{init_idt, interrupt_indexes::RealTime::HardDeadline};

	init_idt();
	unsafe {
		dandelion::software_interrupt!(HardDeadline);
	}

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
