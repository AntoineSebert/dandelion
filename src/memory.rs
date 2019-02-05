/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	05/02/2019
 */

extern crate x86_64;

use self::x86_64::structures::paging::{Mapper, Page, PageTable, RecursivePageTable};
use self::x86_64::{VirtAddr, PhysAddr};

/// Creates a RecursivePageTable instance from the level 4 address.
///
/// This function is unsafe because it can break memory safety if an invalid address is passed.
pub unsafe fn init(level_4_table_addr: usize) -> RecursivePageTable<'static> {
	let level_4_table_ptr = level_4_table_addr as *mut PageTable;
	let level_4_table = &mut *level_4_table_ptr;
	RecursivePageTable::new(level_4_table).unwrap()
}

/// Returns the physical address for the given virtual address, or `None` if the virtual address is not mapped.
pub fn translate_addr(addr: u64, recursive_page_table: &RecursivePageTable) -> Option<PhysAddr> {
	let addr = VirtAddr::new(addr);
	let page: Page = Page::containing_address(addr);

	// perform the translation
	let frame = recursive_page_table.translate_page(page);
	frame.map(|frame| frame.start_address() + u64::from(addr.page_offset()))
}