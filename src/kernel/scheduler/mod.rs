// https://wiki.osdev.org/Loading_a_Process
// https://wiki.osdev.org/Multiprocessor_Scheduling
// https://wiki.osdev.org/Scheduling_Algorithms

pub mod admitter;
pub mod dispatcher;
pub mod swapper;

use super::process::{Constraint, Runnable, State, Task};
use crate::println;
use array_init::array_init;
use arraydeque::{ArrayDeque, CapacityError};
use lazy_static::lazy_static;
use spin::{Mutex, RwLock};

// should be replaced by a set
lazy_static! {
	pub static ref PROCESS_TABLE: [RwLock<Option<Task>>; 256] = { array_init(|_| RwLock::new(None)) };
	pub static ref BLOCKED_QUEUE: Mutex<ArrayDeque<[u8; 256]>> = Mutex::new(ArrayDeque::new());
	pub static ref READY_QUEUE: Mutex<ArrayDeque<[u8; 256]>> = Mutex::new(ArrayDeque::new());
	pub static ref RUNNING: RwLock<Option<u8>> = RwLock::new(None);
	static ref PROCESS_COUNT: RwLock<u8> = RwLock::new(0);
}

/// Run the current running process.
pub fn run() -> Option<u64> {
	let guard = RUNNING.read();

	match *guard {
		Some(index) => {
			let pt_guard = PROCESS_TABLE[usize::from(index)].read();
			let value = pt_guard.as_ref().unwrap().get_runnable().0(&["sample_runnable_2"]);
			drop(pt_guard);
			drop(guard);
			Some(value)
		}
		None => {
			drop(guard);
			None
		}
	}
}

/// Check if a process exists
pub fn process_exists(pid: u8) -> bool {
	let guard = PROCESS_TABLE[pid as usize].read();
	let result = guard.is_some();
	drop(guard);
	result
}

/// Browse PROCESS_TABLE and return the first available slot if it exists.
pub fn get_slot() -> Option<usize> {
	for index in 0..256 {
		let guard = PROCESS_TABLE[index].read();
		if guard.is_none() {
			return Some(index as usize);
		}
		drop(guard);
	}
	None
}

/// Creates a new process and add it ot the PROCESS_TABLE, and stores its index in PROCESS_QUEUE.
fn add_task(constraint: Constraint, code: Runnable, index: usize) {
	let mut guard = PROCESS_TABLE[index].write();
	*guard = Some(Task::new(constraint, code));
	drop(guard);

	increment();

	println!("New process admitted at index {}", index);
}

/// Terminate a job.
/// Returns true if the process exists and has been successfully terminated, false otherwise.
pub fn terminate(pid: u8) -> bool {
	use super::process::{Limbo, MainMemory};

	if process_exists(pid) {
		let mut pt_guard = PROCESS_TABLE[pid as usize].write();
		let state = pt_guard.as_ref().unwrap().get_state();
		pt_guard.as_mut().unwrap().set_state(State::Limbo(Limbo::Terminated));
		match state {
			State::MainMemory(MainMemory::Running) => {
				let guard = RUNNING.read();
				if guard.is_some() && guard.unwrap() == pid {
					let mut wguard = RUNNING.write();
					wguard.take();
					drop(wguard);
				}
				drop(guard);
			}
			State::MainMemory(MainMemory::Ready) => {
				queue_remove(&READY_QUEUE, pid);
			}
			State::SwapSpace(_) => {
				queue_remove(&BLOCKED_QUEUE, pid);
			}
			_ => {}
		};
		*pt_guard = None;
		drop(pt_guard);

		decrement();

		true
	} else {
		false
	}
}

// processes count

pub fn get_process_count() -> u8 {
	let guard = PROCESS_COUNT.read();
	let value = *guard;
	drop(guard);
	value
}

fn increment() -> u8 {
	let mut guard = PROCESS_COUNT.write();
	if (*guard) < 255 {
		(*guard) += 1;
	}
	let new_val = *guard;
	drop(guard);
	new_val
}

fn decrement() -> u8 {
	let mut guard = PROCESS_COUNT.write();
	if 0 < (*guard) {
		(*guard) -= 1;
	}
	let new_val = *guard;
	drop(guard);
	new_val
}

// queues

pub fn queue_push_back(queue: &Mutex<ArrayDeque<[u8; 256]>>, pid: u8, state: State) -> Result<(), CapacityError<u8>> {
	let mut q_guard = queue.lock();

	if !process_exists(pid) || q_guard.contains(&pid) {
		return Ok(());
	}

	let result = q_guard.push_back(pid);
	if result.is_ok() {
		let mut pt_guard = PROCESS_TABLE[pid as usize].write();
		pt_guard.as_mut().unwrap().set_state(state);
		drop(pt_guard);
	}
	drop(q_guard);

	result
}

pub fn queue_remove(queue: &Mutex<ArrayDeque<[u8; 256]>>, pid: u8) -> bool {
	let mut guard = queue.lock();

	for index in 0..guard.len() {
		if (*guard)[index] == pid {
			guard.remove(index);
			drop(guard);
			return true;
		}
	}
	drop(guard);

	false
}

/// Return the size of the queue given as parameter.
pub fn queue_size(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> usize {
	let guard = queue.lock();
	let value = guard.len();
	drop(guard);
	value
}

/// Return an `Option` containing a copy of the first element of the queue if it exists, and `None` otherwise.
pub fn queue_front(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8> {
	let guard = queue.lock();
	let value = if !guard.is_empty() { Some(*(guard.front().unwrap())) } else { None };
	drop(guard);
	value
}
