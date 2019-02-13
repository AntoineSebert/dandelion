/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/02/2019
 */

// crate
extern crate lazy_static;
extern crate x86_64;

// use
use lazy_static::lazy_static;
use x86_64::structures::{gdt, tss::TaskStateSegment};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
	static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();
		tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
			const STACK_SIZE: usize = 4096;
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

			let stack_start = x86_64::VirtAddr::from_ptr(unsafe { &STACK });
			stack_start + STACK_SIZE
		};
		tss
	};
}

lazy_static! {
	static ref GDT: (gdt::GlobalDescriptorTable, Selectors) = {
		use gdt::Descriptor;
		let mut gdt = gdt::GlobalDescriptorTable::new();
		let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
		let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
		(gdt, Selectors { code_selector, tss_selector })
	};
}

struct Selectors {
	code_selector: gdt::SegmentSelector,
	tss_selector: gdt::SegmentSelector,
}

pub fn init() {
	use x86_64::instructions::{segmentation::set_cs, tables::load_tss};

	GDT.0.load();
	unsafe {
		set_cs(GDT.1.code_selector);
		load_tss(GDT.1.tss_selector);
	}
}
