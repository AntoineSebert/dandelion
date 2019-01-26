/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	20/01/2019
 */

// run with : cls && bootimage build && bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none
// bootable USB : dd if=target/x86_64-blog_os/debug/bootimage-dandelion.bin of=/dev/sdX && sync

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

/*
 * This function is called on panic.
 * @param	info	information about the panic error
 */

extern crate x86_64;

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}

/*
 * OS entry point override
 * This function is the entry point, since the linker looks for a function named `_start` by default
 */

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("It's alive {}", "!");

	serial_println!("Hello Host {}", "!");

	unsafe { exit_qemu(); }

	loop {}
}

pub unsafe fn exit_qemu() {
	use x86_64::instructions::port::Port;

	let mut port = Port::<u32>::new(0xf4);
	port.write(0);
}

/*
 Tests
 */

#[cfg(not(feature = "integration-test"))] // new
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello World{}", "!"); // prints to vga buffer

	// normal execution

	loop {}
}

#[cfg(feature = "integration-test")] // new
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	serial_println!("Hello Host{}", "!");

	run_test_1();
	run_test_2();
	// run more tests

	unsafe { exit_qemu(); }

	loop {}
}