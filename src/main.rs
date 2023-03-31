//! run & tests
//!		cargo build
//!		cargo run
//!		cargo test
//!
//! format & lint
//!		cargo fmt && cargo clippy
//!
//! detect certain classes of undefined behavior (currently not working)
//!		cargo clean && cargo miri test && cargo miri run
//!
//! repository data
//!		tokei ./src --files
//!		cargo depgraph | dot -Tpng > graph.png
//!
//! bootable USB
//!		dd if=target/x86_64-dandelion/debug/bootimage-dandelion.bin of=/dev/sdX && sync
//!
//! misc
//!		https://giphy.com/gifs/love-cute-adorable-RExphJPPMEVeo
//!		https://perf.rust-lang.org/
//!		https://rust-lang.github.io/rustup-components-history/

#![no_std]
#![no_main]
#![test_runner(dandelion::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(trait_alias)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]
#![allow(clippy::tabs_in_doc_comments)]

extern crate alloc;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use dandelion::{hlt_loop, kernel, println};

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
	let mut config = BootloaderConfig::new_default();
	config.mappings.physical_memory = Some(Mapping::Dynamic);
	config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG); // OS entry point override.

/// Entry point of the OS.
/// Initialize the kernel components and launch the user space.
/// Infinite loop at the end.
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
	use kernel::task::{executor::Executor, keyboard, Task};

	println!("Hello World!");
	dandelion::init();
	map_memory(boot_info);

	#[cfg(test)]
	test_main();

	user_space();

	let mut executor = Executor::new();
	executor.spawn(Task::new(example_task()));
	executor.spawn(Task::new(keyboard::print_keypresses()));
	executor.run();

	println!("It did not crash!");
	hlt_loop();
}

/// Creates a mapper and a frame allocator.
/// Maps a page corresponding to the screen and writes "New!" into it.
fn map_memory(boot_info: &'static BootInfo) {
	use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
	use kernel::vmm::{
		allocator,
		memory::{create_example_mapping, init, BootInfoFrameAllocator},
	};
	use x86_64::{structures::paging::Page, VirtAddr};

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
	let mut mapper = unsafe { init(phys_mem_offset) };
	let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };

	{
		// map a previously unmapped page
		let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
		create_example_mapping(page, &mut mapper, &mut frame_allocator);

		// write the string `New!` to the screen through the new mapping
		let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
		unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
	}

	// Initialize heap
	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

	{
		let heap_value = Box::new(41);
		println!("heap_value at {heap_value:p}");

		// create a dynamically sized vector
		let mut vec = Vec::new();
		for i in 0..500 {
			vec.push(i);
		}
		println!("vec at {:p}", vec.as_slice());

		// create a reference counted vector -> will be freed when count reaches 0
		let reference_counted = Rc::new(vec![1, 2, 3]);
		let cloned_reference = reference_counted.clone();
		println!("current reference count is {}", Rc::strong_count(&cloned_reference));
		core::mem::drop(reference_counted);
		println!("reference count is {} now", Rc::strong_count(&cloned_reference));
	}
}

/// Schedule and run the user processes.
fn user_space() {
	use kernel::{
		process::{sample_runnable_2, Runnable, PRIORITY::*},
		scheduler::{admitter::request, process_exists, run, terminate},
	};

	println!("process 0 exists ? {}", process_exists(0));
	if let Some(pid) = request((None, MEDIUM), Runnable(sample_runnable_2)) {
		println!("process has pid 0 ? {pid}");
	}
	println!("process 0 exists ? {}", process_exists(0));

	match run() {
		Some(value) => println!("Processed finished with code : {value}"),
		None => println!("No process to run"),
	}

	println!("removing process 0...{}", terminate(0));
	println!("process 0 exists ? {}", process_exists(0));
}

async fn async_number() -> u32 { 42 }

async fn example_task() {
	let number = async_number().await;
	println!("async number: {number}");
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

#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}
