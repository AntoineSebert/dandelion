/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	20/01/2019
 */

/*
run & tests
	cls && bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 && bootimage test

format & lint
	cargo +nightly fmt && cargo +nightly clippy

bootable USB
	dd if=target/x86_64-dandelion/debug/bootimage-dandelion.bin of=/dev/sdX && sync

misc
	https://giphy.com/gifs/love-cute-adorable-RExphJPPMEVeo
	let mortal_heroes: String = "your fame";
*/

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]
#![deny(clippy::all)]
#![feature(asm)]
use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use dandelion::{hlt_loop, println};

/*
 * OS entry point override
 */
entry_point!(kernel_main);

#[cfg(not(test))]
#[allow(clippy::print_literal)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use dandelion::{
		gdt::init_gdt,
		interrupts::{change_real_time_clock_interrupt_rate, enable_rtc_interrupt, init_idt, PICS},
		kernel::vmm::memory::{create_example_mapping, init, init_frame_allocator},
	};
	use x86_64::{instructions::interrupts::enable, structures::paging::Page, VirtAddr};

	println!("Hello World{}", "!");

	init_gdt();
	init_idt();
	unsafe { PICS.lock().initialize() };
	enable();

	let mut mapper = unsafe { init(boot_info.physical_memory_offset) };
	let mut frame_allocator = init_frame_allocator(&boot_info.memory_map);

	// map a previously unmapped page
	let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
	create_example_mapping(page, &mut mapper, &mut frame_allocator);

	// write the string `New!` to the screen through the new mapping
	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

	//sample_job(1_000_000, true);
	enable_rtc_interrupt();
	change_real_time_clock_interrupt_rate(12);

	println!("It did not crash!");
	hlt_loop();
}

/*
 * This function is called on panic.
 * @param	info	information about the panic error
 */
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	hlt_loop();
}

/*
 * Sample job streaming prime numbers on the serial port up to a limit (passed as parameter) less than 2^64
 * On my computer, find all the primes between 0 and 1.000.000 in 2:05 min
 */
#[allow(dead_code)]
fn sample_job(limit: u64, output: bool) {
	use dandelion::{println, serial_println};
	use integer_sqrt::IntegerSquareRoot;

	if output {
		println!("2");
	} else {
		serial_println!("2");
	}
	let mut candidate: u64 = 3;
	loop {
		if limit < candidate {
			break;
		}
		let mut iterator = 3;
		let mut is_prime = true;
		loop {
			if candidate.integer_sqrt() < iterator {
				break;
			}
			if candidate % iterator == 0 {
				is_prime = false;
				break;
			}
			iterator += 2;
		}
		if is_prime {
			if output {
				println!("{}", candidate);
			} else {
				serial_println!("{}", candidate);
			}
		}
		candidate += 2;
	}
}
