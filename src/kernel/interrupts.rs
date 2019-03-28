/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/02/2019
 */

//#![cfg(not(windows))] // fix lint errors, but probably important

use crate::{hlt_loop, print, println};
use interrupt_indexes::{Hardware::*, RealTime::*};
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin::Mutex;
use x86_64::{
	instructions::{interrupts::without_interrupts, port::Port},
	structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

pub const PIC_1_OFFSET: u8 = 32; // 32 to 39
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; // 40 to 47

pub mod interrupt_indexes {
	#[derive(Debug, Clone, Copy)]
	#[repr(u8)]
	pub enum Hardware {
		Timer = super::PIC_1_OFFSET,
		Keyboard,
		Cascade,
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
		TimeRemaining,
		TaskRemaining,
		MissedDeadline,
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

pub fn init() { IDT.load(); }

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		/* software */ {
			use super::vmm::gdt::DOUBLE_FAULT_IST_INDEX;
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
			/*
			idt[Cascade.as_usize()].set_handler_fn(_interrupt_handler);
			idt[SerialPort2.as_usize()].set_handler_fn(_interrupt_handler);
			idt[SerialPort1.as_usize()].set_handler_fn(_interrupt_handler);
			idt[ParallelPort2_3.as_usize()].set_handler_fn(_interrupt_handler);
			idt[FloppyDisk.as_usize()].set_handler_fn(_interrupt_handler);
			idt[ParallelPort1.as_usize()].set_handler_fn(_interrupt_handler);
			*/
			idt[RealTimeClock.as_usize()].set_handler_fn(real_time_clock_interrupt_handler);
			/*
			idt[Acpi.as_usize()].set_handler_fn(_interrupt_handler);
			idt[Available1.as_usize()].set_handler_fn(_interrupt_handler);
			idt[Available2.as_usize()].set_handler_fn(_interrupt_handler);
			idt[Mouse.as_usize()].set_handler_fn(_interrupt_handler);
			idt[CoProcessor.as_usize()].set_handler_fn(_interrupt_handler);
			idt[PrimaryAta.as_usize()].set_handler_fn(_interrupt_handler);
			idt[SecondaryAta.as_usize()].set_handler_fn(_interrupt_handler);
			*/
		}
		/* realtime */ {
			idt[HardDeadline.as_usize()].set_handler_fn(hard_deadline_handler);
			idt[FirmDeadline.as_usize()].set_handler_fn(firm_deadline_handler);
			idt[SoftDeadline.as_usize()].set_handler_fn(soft_deadline_handler);
			idt[TimeRemaining.as_usize()].set_handler_fn(time_remaining_handler);
			idt[TaskRemaining.as_usize()].set_handler_fn(task_remaining_handler);
			idt[MissedDeadline.as_usize()].set_handler_fn(missed_deadline_handler);
		}

		idt
	};
}

/*
 * handlers
 */

// CPU exceptions

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) {
	println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
	hlt_loop();
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: PageFaultErrorCode) {
	use x86_64::registers::control::Cr2;

	println!("EXCEPTION: PAGE FAULT");
	println!("Accessed Address: {:?}", Cr2::read());
	println!("{:#?}", stack_frame);

	hlt_loop();
}

// hardware

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
	// wake up short-term scheduler
	print!(".");
	unsafe { PICS.lock().notify_end_of_interrupt(Timer.as_u8()) }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
	use pc_keyboard::{
		layouts::Us104Key,
		DecodedKey::{RawKey, Unicode},
		Keyboard, ScancodeSet1,
	};

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

extern "x86-interrupt" fn real_time_clock_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
	println!("+");
	without_interrupts(|| {
		/*
		CMOS.lock().write(0x70, 0x0C);
		CMOS.lock().read(0x71);
		*/
		// flush register C so interrupt can happen again
		unsafe {
			Port::<u8>::new(0x70).write(0x0C);
			Port::<u8>::new(0x71).read();
		}
	});

	unsafe { PICS.lock().notify_end_of_interrupt(RealTimeClock.as_u8()) }
}

pub fn change_real_time_clock_interrupt_rate(mut rate: u8) {
	rate &= 0x0F; // rate must be above 2 and not over 15, by default 6
	without_interrupts(|| {
		/*
		CMOS.lock().write(0x70, 0x8A);
		let prev: u8 = CMOS.lock().read(0x71);
		CMOS.lock().write(0x70, 0x8A);
		CMOS.lock().write(0x71, (prev & 0xF0) | rate);
		*/
		let mut address_port = Port::<u8>::new(0x70);
		let mut data_port = Port::<u8>::new(0x71);

		unsafe {
			address_port.write(0x8A);
			let prev: u8 = data_port.read();
			address_port.write(0x8A);
			data_port.write((prev & 0xF0) | rate);
		}
	});

	println!("New frequency is {}", 32768 >> (rate - 1));
}

pub fn enable_rtc_interrupt() {
	without_interrupts(|| {
		/*
		CMOS.lock().write(0x70, 0x8B);
		let prev: u8 = CMOS.lock().read(0x71);
		CMOS.lock().write(0x70, 0x8B);
		CMOS.lock().write(0x71, prev | 0x40);
		*/
		let mut address_port = Port::<u8>::new(0x70);
		let mut data_port = Port::<u8>::new(0x71);

		unsafe {
			address_port.write(0x8B);
			let prev: u8 = data_port.read();
			address_port.write(0x8B);
			data_port.write(prev | 0x40);
		}
	});
}

// realtime

extern "x86-interrupt" fn hard_deadline_handler(stack_frame: &mut InterruptStackFrame) {
	println!("EXCEPTION: HARD DEADLINE\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn firm_deadline_handler(stack_frame: &mut InterruptStackFrame) {
	println!("EXCEPTION: FIRM DEADLINE\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn soft_deadline_handler(stack_frame: &mut InterruptStackFrame) {
	println!("EXCEPTION: SOFT DEADLINE\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn time_remaining_handler(stack_frame: &mut InterruptStackFrame) {
	println!("EXCEPTION: TIME REMAINING\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn task_remaining_handler(stack_frame: &mut InterruptStackFrame) {
	println!("EXCEPTION: TASK REMAINING\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn missed_deadline_handler(stack_frame: &mut InterruptStackFrame) {
	println!("EXCEPTION: MISSED DEADLINE\n{:#?}", stack_frame);
}

/*
uint as error code
argument as power of 2
number between 0 and 1 as estimated remaining time
	-> 0 : 1/2^0 = 1
	-> 1 : 1/2^1 = 0.5
	-> 2 : 1/2^2 = 0.25
*/