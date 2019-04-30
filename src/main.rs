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
	tokei ./src --files
	cargo deps --all-deps | dot -Tpng > graph.png
*/

#![allow(dead_code)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]
#![feature(asm)]
#![feature(trait_alias)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use dandelion::{hlt_loop, kernel, println};
use kernel::{acpi, interrupts, process, scheduler, vmm};
use x86_64::instructions;

// OS entry point override
entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use process::{sample_runnable_2, PRIORITY::*};
	use scheduler::{admitter::request, process_exists, run, terminate};

	println!("Hello World{}", "!");
	initialize_components();
	map_memory(boot_info);

	/* scheduler */
	{
		println!("process 0 exists ? {}", process_exists(0));
		request((None, MEDIUM), sample_runnable_2);
		println!("process 0 exists ? {}", process_exists(0));

		let result = run();
		println!("Processed finished with code : {}", result);

		println!("removing process 0...{}", terminate(0));
		println!("process 0 exists ? {}", process_exists(0));
	}

	println!("It did not crash!");
	hlt_loop();
}

fn map_memory(boot_info: &'static BootInfo) {
	use vmm::memory::{create_example_mapping, init, BootInfoFrameAllocator};
	use x86_64::{structures::paging::Page, VirtAddr};

	let mut mapper = unsafe { init(boot_info.physical_memory_offset) };
	let mut frame_allocator = BootInfoFrameAllocator::init(&boot_info.memory_map);

	// map a previously unmapped page
	let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
	create_example_mapping(page, &mut mapper, &mut frame_allocator);

	// write the string `New!` to the screen through the new mapping
	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
}

fn initialize_components() {
	use interrupts::{change_rtc_interrupt_rate, enable_rtc_interrupt, PICS};
	use vmm::gdt;

	unsafe {
		match acpi::init() {
			Ok(_) => println!("ACPI initialized"),
			Err(_) => println!("Could not initialize ACPI"),
		}
	};

	gdt::init();
	interrupts::init();
	unsafe { PICS.lock().initialize() };
	instructions::interrupts::enable();
	change_rtc_interrupt_rate(15);
	enable_rtc_interrupt();
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
