pub mod allocator;
pub mod gdt;
pub mod memory;

/*
use bitflags::bitflags;

/// VM region protection flags
bitflags! {
	pub struct VMProt: u32 {
		const VM_PROT_READ  = 0b00000001;
		const VM_PROT_WRITE = 0b00000010;
		const VM_PROT_EXEC  = 0b00000100;
		const VM_PROT_RX    = VM_PROT_READ.bits | VM_PROT_EXEC.bits;
		const VM_PROT_RW    = VM_PROT_READ.bits | VM_PROT_WRITE.bits;
		const VM_PROT_RWX   = VM_PROT_READ.bits | VM_PROT_WRITE.bits | VM_PROT_EXEC.bits;
	}
}
*/

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! { panic!("allocation error: {:?}", layout) }
