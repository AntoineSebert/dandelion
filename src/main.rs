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
extern crate bootloader;

use dandelion::println;
use dandelion::memory;
use bootloader::{bootinfo::BootInfo, entry_point};

entry_point!(kernel_main);

/*
 * OS entry point override
 * This function is the entry point, since the linker looks for a function named `_start` by default
 */

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

	memory::create_example_mapping(&mut recursive_page_table, &mut frame_allocator);
	unsafe { (0xdeadbeaf900 as *mut u64).write_volatile(0xf021f077f065f04e) };


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
