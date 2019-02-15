/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/02/2019
 */

//#![cfg(not(windows))] // fix lint errors, but probably important

extern crate lazy_static;
extern crate pc_keyboard;
extern crate pic8259_simple;
extern crate spin;

use crate::{hlt_loop, print, println};
use interrupt_indexes::Hardware::{Keyboard, Timer};
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable, PageFaultErrorCode};

pub const PIC_1_OFFSET: u8 = 32; // 32 to 39
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; // 40 to 47

pub mod interrupt_indexes {
	#[derive(Debug, Clone, Copy)]
	#[repr(u8)]
	pub enum Hardware {
		Timer = super::PIC_1_OFFSET,
		Keyboard,
		Other,
		SerialPort2,
		SerialPort1,
		ParallelPort2_3,
		FloppyDisk,
		ParallelPort1,
		RealTimeClock,
		Acpi,
		Available1,
		Available2,
		Mouse,
		CoProcessor,
		PrimaryAta,
		SecondaryAta,
	}
	#[derive(Debug, Clone, Copy)]
	#[repr(u8)]
	pub enum RealTime {
		HardDeadline = super::PIC_2_OFFSET + 8,
		FirmDeadline,
		SoftDeadline,
		TimeRemaining, // with integer as power of 2 as code
		WorkRemaining, // with integer as power of 2 as code
		Periodic,      // with integer time interval as code
		Aperiodic,
	}
	impl Hardware {
		#[inline]
		pub fn as_u8(self) -> u8 { self as u8 }

		#[inline]
		pub fn as_usize(self) -> usize { usize::from(self.as_u8()) }
	}
	impl RealTime {
		#[inline]
		pub fn as_u8(self) -> u8 { self as u8 }

		#[inline]
		pub fn as_usize(self) -> usize { usize::from(self.as_u8()) }
	}
}

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init_idt() { IDT.load(); }

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		/* software */ {
			use crate::gdt::DOUBLE_FAULT_IST_INDEX;
			/*
			idt.divide_by_zero.set_handler_fn();
			idt.debug.set_handler_fn();
			idt.non_maskable_interrupt.set_handler_fn();
			*/
			idt.breakpoint.set_handler_fn(breakpoint_handler);
			/*
			idt.overflow.set_handler_fn();
			idt.bound_range_exceeded.set_handler_fn();
			idt.invalid_opcode.set_handler_fn();
			idt.device_not_available.set_handler_fn();
			*/
			unsafe {
				idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(DOUBLE_FAULT_IST_INDEX);
			}
			/*
			idt.invalid_tss.set_handler_fn();
			idt.segment_not_present.set_handler_fn();
			idt.stack_segment_fault.set_handler_fn();
			idt.general_protection_fault.set_handler_fn();
			*/
			idt.page_fault.set_handler_fn(page_fault_handler);
			/*
			idt.reserved_1.set_handler_fn();
			idt.x87_floating_point.set_handler_fn();
			idt.alignment_check.set_handler_fn();
			idt.machine_check.set_handler_fn();
			idt.simd_floating_point.set_handler_fn();
			idt.virtualization.set_handler_fn();
			idt.reserved_2.set_handler_fn();
			idt.security_exception.set_handler_fn();
			idt.reserved_3.set_handler_fn();
			*/
		}
		/* hardware */ {
			idt[Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
			idt[Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
		}

		idt
	};
}

/*
 * handlers
 */

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
	println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
	hlt_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
	print!(".");
	unsafe { PICS.lock().notify_end_of_interrupt(Timer.as_u8()) }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
	use pc_keyboard::{
		layouts::Us104Key,
		DecodedKey::{RawKey, Unicode},
		Keyboard, ScancodeSet1,
	};
	use x86_64::instructions::port::Port;

	lazy_static! {
		static ref KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> =
			Mutex::new(Keyboard::new(Us104Key, ScancodeSet1));
	}

	let mut keyboard = KEYBOARD.lock();
	let port = Port::new(0x60);

	let scancode: u8 = unsafe { port.read() };
	if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
		if let Some(key) = keyboard.process_keyevent(key_event) {
			match key {
				Unicode(character) => print!("{}", character),
				RawKey(key) => print!("{:?}", key),
			}
		}
	}
	unsafe { PICS.lock().notify_end_of_interrupt(Keyboard.as_u8()) }
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: PageFaultErrorCode) {
	use x86_64::registers::control::Cr2;

	println!("EXCEPTION: PAGE FAULT");
	println!("Accessed Address: {:?}", Cr2::read());
	println!("{:#?}", stack_frame);
	hlt_loop();
}
