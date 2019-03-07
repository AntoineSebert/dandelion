/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	27/01/2019
 */

#![cfg_attr(not(test), no_std)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

pub mod gdt;
pub mod interrupts;
pub mod kernel;
pub mod serial;
pub mod vga_buffer;

use x86_64::instructions::{hlt, port::Port};

/// safely exit the emulator
pub unsafe fn exit_qemu() {
	let mut port = Port::<u32>::new(0xf4);
	port.write(0);
}

///
pub fn hlt_loop() -> ! {
	loop {
		hlt();
	}
}

// environment variables ?
