/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	20/01/2019
 */

/*
run
	cls && bootimage build && bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none
tests
	bootimage test
bootable USB
	dd if=target/x86_64-dandelion/debug/bootimage-dandelion.bin of=/dev/sdX && sync
*/

/*
 * #[pure] : pure function
 * #[] : no side effects
*/

// configuration
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

extern crate dandelion;
extern crate pic8259_simple;
extern crate x86_64;

// modules
mod serial;
mod vga_buffer;

/*
 * OS entry point override
 * This function is the entry point, since the linker looks for a function named `_start` by default
 */

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	use dandelion::interrupts::PICS;

	println!("Hello World{}", "!");

	dandelion::gdt::init();
	dandelion::interrupts::init_idt();
	unsafe { PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();

	 use dandelion::memory::{self, translate_addr};

	const LEVEL_4_TABLE_ADDR: usize = 0o_177777_777_777_777_777_0000;
	let recursive_page_table = unsafe { memory::init(LEVEL_4_TABLE_ADDR) };

	// the identity-mapped vga buffer page
	println!("0xb8000 -> {:?}", translate_addr(0xb8000, &recursive_page_table));
	// some code page
	println!("0x20010a -> {:?}", translate_addr(0x20010a, &recursive_page_table));
	// some stack page
	println!("0x57ac001ffe48 -> {:?}", translate_addr(0x57ac001ffe48, &recursive_page_table));

	println!("It did not crash!");
	dandelion::hlt_loop();
}

/*
 * This function is called on panic.
 * @param	info	information about the panic error
 */

use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	dandelion::hlt_loop();
}
