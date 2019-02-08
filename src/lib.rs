/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	27/01/2019
 */

// configuration
#![cfg_attr(not(test), no_std)]
#![feature(abi_x86_interrupt)]
#[cfg(not(windows))]
// crates
extern crate x86_64;

// modules
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

// functions
pub unsafe fn exit_qemu() {
	use x86_64::instructions::port::Port;

	let mut port = Port::<u32>::new(0xf4);
	port.write(0);
}

pub fn hlt_loop() -> ! {
	loop {
		x86_64::instructions::hlt();
	}
}
