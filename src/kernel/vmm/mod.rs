pub mod allocator;
pub mod gdt;
pub mod memory;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
