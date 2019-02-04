/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/02/2019
 */

#![cfg(not(windows))]

extern crate lazy_static;

use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
use crate::println;
use self::lazy_static::lazy_static;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		idt.double_fault.set_handler_fn(double_fault_handler);
		idt
	};
}

pub fn init_idt() {
	IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
	println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
	loop {}
}

use crate::gdt;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		unsafe {
			idt.double_fault.set_handler_fn(double_fault_handler)
				.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
		}

		idt
	};
}