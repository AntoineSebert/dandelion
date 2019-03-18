/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

#![allow(dead_code)]

use crate::kernel::CMOS;
use cmos::{CMOSCenturyHandler, RTCDateTime};
use core::{hash::{Hash, Hasher}, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
	Limbo(Limbo),
	MainMemory(MainMemory),
	SwapSpace(SwapSpace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Limbo {
	Creating,
	Killed,
	Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMemory {
	Ready,
	Running,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapSpace {
	Interrupted,
	Suspended,
	Delayed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constraint {
	Periodic,
	Aperiodic,
	None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data {
	/// Stack contains all the data that is local to a function like variables, pointers etc. Each function has its own
	/// stack. Stack memory is dynamic in the sense that it grows with each function being called.
	stack_address: usize,
	/// Heap segment contains memory that is dynamically requested by the programs for their variables
	heap_address: usize,
	/// All the global and static members become part of this segment
	data_address: usize,
	/// All the program instructions, hard-coded strings, constant values are a part of this memory area
	text_address: usize,
}
impl Data {
	pub fn default() -> Data { Data { data_address: 0, heap_address: 0, stack_address: 0, text_address: 0 } }
}

#[derive(Debug)]
pub struct Metadata {
	process_id: u64,
	parent_process_id: u64,
	state: State,
	virtual_time: Duration,
	creation_time: RTCDateTime,
	constraint: Constraint,
}
impl PartialEq for Metadata {
	fn eq(&self, other: &Metadata) -> bool { self.process_id == other.process_id }
}
impl Eq for Metadata {}
impl Ord for Metadata {
	fn cmp(&self, other: &Metadata) -> core::cmp::Ordering { self.process_id.cmp(&other.process_id) }
}
impl PartialOrd for Metadata {
	fn partial_cmp(&self, other: &Metadata) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Copy for Metadata {}
impl Clone for Metadata {
	// TODO
	fn clone(&self) -> Self { *self }
}
impl Metadata {
	pub fn default() -> Metadata {
		Metadata {
			process_id: 1,
			parent_process_id: 1,
			state: State::Limbo(Limbo::Creating),
			virtual_time: Duration::from_secs(0),
			creation_time: CMOS.lock().read_rtc(CMOSCenturyHandler::CurrentYear(2019)),
			constraint: Constraint::None,
		}
	}
}

pub struct Runnable {
	data: Data,
}

#[derive(Debug)]
pub struct Task {
	data: Data,
	metadata: Metadata,
}
impl Hash for Task {
	fn hash<H: Hasher>(&self, state: &mut H) { self.metadata.process_id.hash(state); }
}
impl Copy for Task {}
impl Clone for Task {
	// TODO
	fn clone(&self) -> Self { *self }
}
impl Task {
	pub fn default() -> Task { Task { data: Data::default(), metadata: Metadata::default() } }
}

pub struct Job {
	metadata: Metadata,
	workers: [Runnable; 256],
}

pub struct Group {
	tasks: [Task; 256],
}

/*
not good enough to imlement this

process<
	either<
		collection,
		metadata
	>
> (opt<
	meta + either<
		Periodic,
		datetime,
		none
	>
>)

if array + meta : job
if data + meta : task
*/
