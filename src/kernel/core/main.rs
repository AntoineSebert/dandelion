/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	20/01/2019
 */

// compile with : cargo xbuild --target x86_64-dandelion.json
// run with : qemu-system-x86_64 -drive format=raw,file=bootimage-dandelion.bin
// bootable USB : dd if=target/x86_64-blog_os/debug/bootimage-dandelion.bin of=/dev/sdX && sync

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/* This function is called on panic.
 * @param	_info	information about the panic error
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

static NEXUS6: &[u8] = b"All those moments will be lost in time, like tears in rain.";

/* OS entry point override
 * This function is the entry point, since the linker looks for a function named `_start` by default
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {
	let vga_buffer = 0xb8000 as *mut u8;

	for (i, &byte) in NEXUS6.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xa;
		}
	}

	loop {}
}