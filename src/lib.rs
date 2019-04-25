/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	27/01/2019
 */

#![cfg_attr(not(test), no_std)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(trait_alias)]
#![feature(core_intrinsics)]

pub mod kernel;

use x86_64::instructions::{hlt, port::Port};

/// safely exit the emulator
pub unsafe fn exit_qemu() {
	let mut port = Port::<u32>::new(0xf4);
	port.write(0);
}

/// brøther may I have some lööps
pub fn hlt_loop() -> ! {
	loop {
		hlt();
	}
}

// environment variables ?
