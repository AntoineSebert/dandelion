/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/02/2019
 */

extern crate x86_64;
extern crate lazy_static;

use self::x86_64::VirtAddr;
use self::x86_64::structures::tss::TaskStateSegment;
use self::lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
	static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();
		tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
			const STACK_SIZE: usize = 4096;
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

			let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
			let stack_end = stack_start + STACK_SIZE;
			stack_end
		};
		tss
	};
}

use self::x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use self::x86_64::structures::gdt::SegmentSelector;

lazy_static! {
	static ref GDT: (GlobalDescriptorTable, Selectors) = {
		let mut gdt = GlobalDescriptorTable::new();
		let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
		let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
		(gdt, Selectors { code_selector, tss_selector })
	};
}

struct Selectors {
	code_selector: SegmentSelector,
	tss_selector: SegmentSelector,
}

pub fn init() {
	use self::x86_64::instructions::segmentation::set_cs;
	use self::x86_64::instructions::tables::load_tss;

	GDT.0.load();
	unsafe {
		set_cs(GDT.1.code_selector);
		load_tss(GDT.1.tss_selector);
	}
}