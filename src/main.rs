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

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]
#![deny(clippy::all)]
#![feature(asm)]
#![feature(trait_alias)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]

use bootloader::{bootinfo::BootInfo, entry_point};
use core::{
	alloc::{GlobalAlloc, Layout},
	panic::PanicInfo,
	ptr::null_mut,
};
use dandelion::{hlt_loop, kernel, println};
use x86_64::instructions::interrupts;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
	unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut() }

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static A: MyAllocator = MyAllocator;

// OS entry point override
entry_point!(kernel_main);

#[cfg(not(test))]
#[allow(clippy::print_literal)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use kernel::{
		process::{sample_runnable_2, PRIORITY::*},
		scheduler::{admitter::request, process_exists, run, terminate},
	};

	println!("Hello World{}", "!");
	initialize_components();
	unsafe { assert!(A.alloc(Layout::new::<u32>()).is_null()) };
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

#[allow(dead_code)]
fn map_memory(boot_info: &'static BootInfo) {
	use kernel::vmm::memory::{create_example_mapping, init, init_frame_allocator};
	use x86_64::{structures::paging::Page, VirtAddr};

	let mut mapper = unsafe { init(boot_info.physical_memory_offset) };
	let mut frame_allocator = init_frame_allocator(&boot_info.memory_map);

	// map a previously unmapped page
	let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
	create_example_mapping(page, &mut mapper, &mut frame_allocator);

	// write the string `New!` to the screen through the new mapping
	let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
	unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
}

#[allow(dead_code)]
fn initialize_components() {
	use kernel::{
		acpi,
		interrupts::{change_rtc_interrupt_rate, enable_rtc_interrupt, PICS},
		vmm::gdt,
	};

	unsafe {
		match acpi::init() {
			Ok(_) => println!("ACPI initialized"),
			Err(_) => println!("Could not initialize ACPI"),
		}
	};

	gdt::init();
	kernel::interrupts::init();
	unsafe { PICS.lock().initialize() };
	interrupts::enable();
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
