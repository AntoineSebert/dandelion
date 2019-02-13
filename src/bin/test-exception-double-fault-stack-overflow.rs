/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/02/2019
 */

// configuration
#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

// crate
extern crate dandelion;
extern crate lazy_static;
extern crate x86_64;

// use
use dandelion::{exit_qemu, gdt, serial_println};
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

#[cfg(not(test))]
#[no_mangle]
#[allow(unconditional_recursion)]
pub extern "C" fn _start() -> ! {
	gdt::init();
	init_test_idt();

	fn stack_overflow() {
		stack_overflow(); // for each recursion, the return address is pushed
	}

	stack_overflow(); // trigger a stack overflow

	serial_println!("failed");
	serial_println!("No exception occured");

	unsafe {
		exit_qemu();
	}
	loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	serial_println!("failed");
	serial_println!("{}", info);

	unsafe {
		exit_qemu();
	}
	loop {}
}

lazy_static::lazy_static! {
	static ref TEST_IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		unsafe {
			idt.double_fault
				.set_handler_fn(double_fault_handler)
				.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}

		idt
	};
}

pub fn init_test_idt() { TEST_IDT.load(); }

extern "x86-interrupt" fn double_fault_handler(_stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
	serial_println!("ok");

	unsafe {
		exit_qemu();
	}
	loop {}
}