// https://wiki.osdev.org/Loading_a_Process
// https://wiki.osdev.org/Multiprocessor_Scheduling
// https://wiki.osdev.org/Scheduling_Algorithms

pub mod admitter;
pub mod dispatcher;
pub mod swapper;

use super::process::{task::Task, Aperiodic, Constraint, Periodic, Runnable, State, PRIORITY};
use crate::println;
use alloc::{string::String, vec};
use array_init::array_init;
use arraydeque::{ArrayDeque, CapacityError};
use core::u8::MAX;
use either::Either;
use lazy_static::lazy_static;
use spin::{Mutex, RwLock};

lazy_static! {
	pub static ref PROCESS_TABLE: [RwLock<Option<Task>>; 256] = { array_init(|_| RwLock::new(None)) }; // should be replaced by a set
	pub static ref BLOCKED_QUEUE: Mutex<ArrayDeque<[u8; 256]>> = Mutex::new(ArrayDeque::new());
	pub static ref READY_QUEUE: Mutex<ArrayDeque<[u8; 256]>> = Mutex::new(ArrayDeque::new());
	pub static ref RUNNING: RwLock<Option<u8>> = RwLock::new(None);
	static ref PROCESS_COUNT: RwLock<u8> = RwLock::new(0);
}

/// Run the current running process.
pub fn run() -> Option<u64> {
	match *(RUNNING.read()) {
		Some(index) => {
			Some(PROCESS_TABLE[usize::from(index)].read().as_ref().unwrap().get_runnable().0(vec![String::from(
				"sample_runnable_2",
			)]))
		}
		None => None,
	}
}

/// Check if a process exists
pub fn process_exists(pid: u8) -> bool { PROCESS_TABLE[pid as usize].read().is_some() }

/// Browse PROCESS_TABLE and return the first available slot if it exists.
pub fn get_slot() -> Option<usize> {
	for (index, element) in PROCESS_TABLE.iter().enumerate() {
		if element.read().is_none() {
			return Some(index);
		}
	}
	None
}

/// Set the state of a process.
/// Return true is the operation could be performed, and false otherwise.
pub fn set_process_state(pid: u8, state: State) -> bool {
	match PROCESS_TABLE[pid as usize].write().as_mut() {
		Some(process) => {
			process.set_state(state);
			true
		}
		None => false,
	}
}

/// Returns the periodicity of a process.
/// If the process is non-realtime or does not exists, the result is `None`.
pub fn get_process_periodicity(pid: u8) -> Option<Either<Periodic, Aperiodic>> {
	match PROCESS_TABLE[pid as usize].write().as_mut() {
		Some(process) => *process.get_periodicity(),
		None => None,
	}
}

/// Returns an `Option` containing a `PRIORITY` if the process exists, or `None` if it does not.
pub fn get_process_priority(pid: u8) -> Option<PRIORITY> {
	match PROCESS_TABLE[pid as usize].write().as_mut() {
		Some(process) => Some(process.get_priority()),
		None => None,
	}
}

/// Creates a new process and add it ot the PROCESS_TABLE, and stores its index in PROCESS_QUEUE.
fn add_task(constraint: Constraint, code: Runnable, index: usize) {
	*(PROCESS_TABLE[index].write()) = Some(Task::new(constraint, code));
	increment();
	println!("New process admitted at index {}", index);
}

/// Terminate a job.
/// Returns true if the process exists and has been successfully terminated, false otherwise.
pub fn terminate(pid: u8) -> bool {
	use super::process::{Limbo, MainMemory};
	use swapper::get_running;

	if process_exists(pid) {
		let mut pt_guard = PROCESS_TABLE[pid as usize].write();
		pt_guard.as_mut().unwrap().set_state(State::Limbo(Limbo::Terminated));
		match pt_guard.as_ref().unwrap().get_state() {
			State::MainMemory(MainMemory::Running) => {
				if let Some(running_pid) = get_running() {
					if running_pid == pid {
						RUNNING.write().take();
					}
				}
			}
			State::MainMemory(MainMemory::Ready) => {
				queue_remove(&READY_QUEUE, pid);
			}
			State::SwapSpace(_) => {
				queue_remove(&BLOCKED_QUEUE, pid);
			}
			State::Limbo(_) => {}
		};
		*pt_guard = None;

		decrement();

		true
	} else {
		false
	}
}

// processes count

pub fn get_process_count() -> u8 { *PROCESS_COUNT.read() }

fn increment() -> u8 {
	let mut guard = PROCESS_COUNT.write();
	if *guard < MAX {
		*guard += 1;
	}
	*guard
}

fn decrement() -> u8 {
	let mut guard = PROCESS_COUNT.write();
	if 0 < *guard {
		*guard -= 1;
	}
	*guard
}

// queues

pub fn queue_push_back(queue: &Mutex<ArrayDeque<[u8; 256]>>, pid: u8, state: State) -> Result<(), CapacityError<u8>> {
	let mut q_guard = queue.lock();

	if !process_exists(pid) || q_guard.contains(&pid) {
		Ok(())
	} else {
		let result = q_guard.push_back(pid);
		if result.is_ok() {
			PROCESS_TABLE[pid as usize].write().as_mut().unwrap().set_state(state);
		}

		result
	}
}

pub fn queue_remove(queue: &Mutex<ArrayDeque<[u8; 256]>>, pid: u8) -> bool {
	let mut guard = queue.lock();

	for index in 0..guard.len() {
		if guard.get(index) == Some(&pid) {
			guard.remove(index);
			return true;
		}
	}

	false
}

/// Return the size of the queue given as parameter.
pub fn queue_size(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> usize { queue.lock().len() }

/// Return an `Option` containing a copy of the first element of the queue if it exists, and `None` otherwise.
pub fn queue_front(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8> {
	let guard = queue.lock();

	if !guard.is_empty() {
		Some(*(guard.front().unwrap()))
	} else {
		None
	}
}
