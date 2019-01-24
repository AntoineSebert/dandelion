/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	20/01/2019
 */

// compile with : bootimage build
// run with : bootimage run
// bootable USB : dd if=target/x86_64-blog_os/debug/bootimage-dandelion.bin of=/dev/sdX && sync

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

/*
 * This function is called on panic.
 * @param	info	information about the panic error
 */

mod vga_buffer;

use core::panic::PanicInfo;
//use self::println;

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
	println!("It's alive !{}", "!");

	loop {}
}