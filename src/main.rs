//! target file argument (sometimes needed)
//!		--target x86_64-dandelion.json
//!
//! run & tests
//!		cargo xrun
//!		cargo xtest
//!
//! format & lint
//!		cargo +nightly fmt && cargo +nightly clippy
//!
//! repository data
//!		tokei ./src --files
//!		cargo deps --all-deps | dot -Tpng > graph.png
//!
//! bootable USB
//!		dd if=target/x86_64-dandelion/debug/bootimage-dandelion.bin of=/dev/sdX && sync
//!
//! misc
//!		https://giphy.com/gifs/love-cute-adorable-RExphJPPMEVeo
//!		let mortal_heroes: String = "your fame";
//!		https://perf.rust-lang.org/

#![no_std]
#![no_main]
#![test_runner(dandelion::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)]
#![feature(trait_alias)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use dandelion::{hlt_loop, kernel, println};

entry_point!(kernel_main); // OS entry point override.

/// Entry point of the OS.
/// Initialize the kernel components and launch the user space.
/// Infinite loop at the end.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	println!("Hello World{}", "!");
	dandelion::init();
	map_memory(boot_info);

	#[cfg(test)]
	test_main();

	user_space();

	println!("It did not crash!");
	hlt_loop();
}

/// Schedule and run the user processes.
fn user_space() {
	use kernel::{
		process::{sample_runnable_2, PRIORITY::*},
		scheduler::{admitter::request, process_exists, run, terminate},
	};

	println!("process 0 exists ? {}", process_exists(0));
	request((None, MEDIUM), sample_runnable_2);
	println!("process 0 exists ? {}", process_exists(0));

	match run() {
		Some(value) => println!("Processed finished with code : {}", value),
		None => println!("No process to run"),
	}

	println!("removing process 0...{}", terminate(0));
	println!("process 0 exists ? {}", process_exists(0));
}

/// Creates a mapper and a frame allocator.
/// Maps a page corresponding to the screen and writes "New!" into it.
fn map_memory(boot_info: &'static BootInfo) {
	use kernel::vmm::memory::{create_example_mapping, init, BootInfoFrameAllocator};
	use x86_64::{structures::paging::Page, VirtAddr};

	let mut mapper = unsafe { init(boot_info.physical_memory_offset) };
	let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

	// map a previously unmapped page
	let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
	create_example_mapping(page, &mut mapper, &mut frame_allocator);

	// write the string `New!` to the screen through the new mapping
	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
}

/// Called on panic and prints information about the panic error.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	hlt_loop();
}

/// Panic handler for testing purposes, calls `test_panic_handler(&PanicInfo)`.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { dandelion::test_panic_handler(info) }
