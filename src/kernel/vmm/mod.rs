pub mod allocator;
pub mod gdt;
pub mod memory;

/*

/// VM region protection flags
bitflags! {
}
*/

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! { panic!("allocation error: {:?}", layout) }
