/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
	Limbo(Limbo),
	MainMemory(MainMemory),
	SwapSpace(SwapSpace),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Limbo {
	Creating,
	Killed,
	Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainMemory {
	Ready,
	Running,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwapSpace {
	Interrupted,
	Suspended,
	Delayed,
}

// process
#[derive(Debug, Clone, Copy)]
pub struct Process {
	process_id: u64,
	parent_process_id: u64,
	state: ProcessState,
	virtual_time: u64, // execution time elapsed
	creation_time: u64,
	stack_address: usize, /* Stack contains all the data that is local to a function like variables, pointers etc.
	                       * Each function has its own stack. Stack memory is dynamic in the sense that it grows
	                       * with each function being called. */
	heap_address: usize, /* Heap segment contains memory that is dynamically requested by the programs for their
	                      * variables. */
	data_address: usize, // All the global and static members become part of this segment.
	text_address: usize, /* All the program instructions, hard-coded strings, constant values are a part of this
	                      * memory area. */
}

impl Process {
	pub fn spawn_process(parent_process_id: u64) -> Process {
		Process {
			process_id: /*process_table.generate_new()*/0,
			parent_process_id,
			state: ProcessState::Limbo(Limbo::Creating),
			virtual_time: 0,
			creation_time: /*::kernel::time::monotonic()*/0,
			stack_address: /*vmm::get_new_stack_address()*/0,
			heap_address: /*vmm::get_new_heap_address()*/0,
			data_address: /*vmm::get_new_data_address()*/0,
			text_address: /*vmm::get_new_text_address()*/0,
		}
	}
}

// process table (set)