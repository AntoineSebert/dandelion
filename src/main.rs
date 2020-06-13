//! run & tests
//!		cargo xrun
//!		cargo xtest
//!
//! format & lint
//!		cargo fmt && cargo +nightly xclippy
//!
//! detect certain classes of undefined behavior (currently not working)
//!		cargo clean && cargo miri test && cargo miri run
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

extern crate alloc;

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use dandelion::{hlt_loop, kernel, println};

entry_point!(kernel_main); // OS entry point override.

/// Entry point of the OS.
/// Initialize the kernel components and launch the user space.
/// Infinite loop at the end.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use kernel::task::{executor::Executor, keyboard, Task};

	println!("Hello World!");
	dandelion::init();
	map_memory(boot_info);

	#[cfg(test)]
	test_main();

	let mut executor = Executor::new();
	executor.spawn(Task::new(example_task()));
	executor.spawn(Task::new(keyboard::print_keypresses()));
	executor.run();

	user_space();

	println!("It did not crash!");
	hlt_loop();
}

/// Creates a mapper and a frame allocator.
/// Maps a page corresponding to the screen and writes "New!" into it.
#[allow(clippy::unreadable_literal)]
fn map_memory(boot_info: &'static BootInfo) {
	use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
	use kernel::vmm::{
		allocator,
		memory::{create_example_mapping, init, BootInfoFrameAllocator},
	};
	use x86_64::{structures::paging::Page, VirtAddr};

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { init(phys_mem_offset) };
	let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

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
		println!("heap_value at {:p}", heap_value);

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
	request((None, MEDIUM), Runnable(sample_runnable_2));
	println!("process 0 exists ? {}", process_exists(0));

	match run() {
		Some(value) => println!("Processed finished with code : {}", value),
		None => println!("No process to run"),
	}

	println!("removing process 0...{}", terminate(0));
	println!("process 0 exists ? {}", process_exists(0));
}

async fn async_number() -> u32 { 42 }

async fn example_task() {
	let number = async_number().await;
	println!("async number: {}", number);
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
