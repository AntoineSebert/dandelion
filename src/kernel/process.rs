/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

#![allow(dead_code)]

use core::option::Option;
use cmos::{RTCDateTime, CMOSCenturyHandler::CurrentYear};
use crate::kernel::CMOS;

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

/*
process<
	either<
		collection,
		metadata
	>
> (opt<
	meta + either<
		interval,
		datetime,
		none
	>
>)

if array + meta : job
if data + meta : task
*/

pub type Interval = u32;
pub type Deadline = u64;
pub type None = ();

pub trait Constraint {}

impl Constraint for Interval {}
impl Constraint for Deadline {}
impl Constraint for None {}

pub type Atom = ();
pub type Collection = [Atom; 256];

pub trait Nature {}

impl Nature for Collection {}
impl Nature for Atom {}

// #[derive(...)]
pub struct Data {
	stack_address: usize, /* Stack contains all the data that is local to a function like variables, pointers etc.
	                       * Each function has its own stack. Stack memory is dynamic in the sense that it grows
	                       * with each function being called. */
	heap_address: usize, /* Heap segment contains memory that is dynamically requested by the programs for their
	                      * variables. */
	data_address: usize, // All the global and static members become part of this segment.
	text_address: usize, /* All the program instructions, hard-coded strings, constant values are a part of this
	                      * memory area. */
}

impl Data {
	pub fn data() -> Data {
		Data {
			stack_address: 0,
			heap_address: 0,
			data_address: 0,
			text_address: 0,
		}
	}
}

/* REFACTOR */

// #[derive(...)]
pub struct Metadata<T: Constraint> {
	process_id: u64,
	parent_process_id: u64,
	state: ProcessState,
	virtual_time: u64, // execution time elapsed
	creation_time: RTCDateTime,
	constraint: T,
}

impl Metadata<Interval> {
	pub fn metadata(_parent_process_id: Option<u64>, _interval: Interval) -> Metadata<Interval> {
		Metadata {
			process_id: 0,											// process_table.get_new_pid()
			parent_process_id: _parent_process_id.unwrap_or(1),		// find a way to get value in _parent_process_id
			state: ProcessState::Limbo(Limbo::Creating),
			virtual_time: 0,
			creation_time: CMOS.lock().read_rtc(CurrentYear(2019)),	// time()
			constraint: _interval,
		}
	}
}

impl Metadata<Deadline> {
	pub fn metadata(_parent_process_id: Option<u64>, _deadline: Deadline) -> Metadata<Deadline> {
		Metadata {
			process_id: 0,											// process_table.get_new_pid()
			parent_process_id: _parent_process_id.unwrap_or(1),		// find a way to get value in _parent_process_id
			state: ProcessState::Limbo(Limbo::Creating),
			virtual_time: 0,
			creation_time: CMOS.lock().read_rtc(CurrentYear(2019)),	// time()
			constraint: _deadline,
		}
	}
}

impl Metadata<None> {
	pub fn metadata(_parent_process_id: Option<u64>) -> Metadata<None> {
		Metadata {
			process_id: 0,											// process_table.get_new_pid()
			parent_process_id: _parent_process_id.unwrap_or(1),		// find a way to get value in _parent_process_id
			state: ProcessState::Limbo(Limbo::Creating),
			virtual_time: 0,
			creation_time: CMOS.lock().read_rtc(CurrentYear(2019)),	// time()
			constraint: (),
		}
	}
}

/* INCOMPLETE */

#[derive(Debug, Clone, Copy)]
pub struct Process<T: Nature> {
	content: T,
}

impl Process<Atom> {
	fn spawn_process(metadata: Option<Metadata<Interval>>)/* -> Process<Atom>*/ {
		if metadata.is_some() {
			// task
			//Process<Atom> {}
		}
		else {
			// runnable
			//Process<Atom> {}
		}
	}
}

impl Process<Collection> {
	fn spawn_process(metadata: Option<Metadata<Interval>>)/* -> Process<Collection>*/ {
		if metadata.is_some() {
			// job
			//Process<Collection> {}
		}
		else {
			// group
			//Process<Collection> {}
		}
	}
}
/*
impl PartialEq for Process {
	fn eq(&self, other: &Process) -> bool { self.process_id == other.process_id }
}

impl Eq for Process {}
*/