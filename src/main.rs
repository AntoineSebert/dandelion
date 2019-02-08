/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	20/01/2019
 */

/*
run
	cls && bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04
tests
	cls && bootimage test
both
	cls && bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 && bootimage test
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

// crates
extern crate bootloader;
extern crate dandelion;
extern crate integer_sqrt;
extern crate pic8259_simple;
extern crate x86_64;

// uses
use bootloader::{bootinfo::BootInfo, entry_point};
use dandelion::{println, memory};

/*
 * OS entry point override
 * This function is the entry point, since the linker looks for a function named `_start` by default
 */

entry_point!(kernel_main);

#[cfg(not(test))]
#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use dandelion::interrupts::PICS;

	println!("Hello World{}", "!");

	dandelion::gdt::init();
	dandelion::interrupts::init_idt();
	unsafe { PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();

	let mut recursive_page_table = unsafe { memory::init(boot_info.p4_table_addr as usize) };
	let mut frame_allocator = memory::init_frame_allocator(&boot_info.memory_map);

	sample_job(4294967296);

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

/*
 * Sample job streaming prime numbers up to 2^64
 */
fn sample_job(limit: u64) {
	use integer_sqrt::IntegerSquareRoot;
	println!("2");
	let mut counter: u64 = 3;
	loop {
		if limit < counter {
			break;
		}
		let mut counter2 = 3;
		let mut is_prime = true;
		loop {
			if counter.integer_sqrt() < counter2 {
				break;
			}
			if counter % counter2 == 0 {
				is_prime = false;
				break;
			}
			counter2 += 2;
		}
		if is_prime {
			println!("{}", counter);
		}
		counter += 2;
	}
}
