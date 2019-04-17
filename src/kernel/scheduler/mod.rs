/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

// https://wiki.osdev.org/Loading_a_Process
// https://wiki.osdev.org/Multiprocessor_Scheduling
// https://wiki.osdev.org/Scheduling_Algorithms

pub mod admitter;
pub mod dispatcher;
pub mod swapper;

use super::process::*;
use array_init::array_init;
use arraydeque::{ArrayDeque, CapacityError};
use lazy_static::lazy_static;
use spin::{Mutex, RwLock};

// should be replaced by a set
lazy_static! {
	pub static ref PROCESS_TABLE: [RwLock<Option<Task>>; 256] = { array_init(|_| RwLock::new(None)) };
}

lazy_static! {
	pub static ref BLOCKED_QUEUE: Mutex<ArrayDeque<[u8; 256]>> = Mutex::new(ArrayDeque::new());
}

lazy_static! {
	pub static ref READY_QUEUE: Mutex<ArrayDeque<[u8; 256]>> = Mutex::new(ArrayDeque::new());
}

lazy_static! {
	pub static ref RUNNING: RwLock<Option<u8>> = RwLock::new(None);
}

lazy_static! {
	pub static ref PROCESS_COUNT: RwLock<u8> = RwLock::new(0);
}

/// Run the current running process
pub fn run() -> u64 {
	use crate::println;

	let guard = RUNNING.read();
	if (*guard).is_none() {
		println!("No process to run");
		drop(guard);
		return 0;
	}
	let index = usize::from((*guard).unwrap());
	drop(guard);
	let guard = PROCESS_TABLE[index].read();
	let result = if (*guard).is_none() {
		println!("No process to run");
		0
	} else {
		println!("Running...");
		guard.as_ref().unwrap().1(&["sample_runnable_2"])
	};
	drop(guard);

	result
}

/// Check if a process exists
pub fn process_exists(pid: u8) -> bool {
	let guard = PROCESS_TABLE[pid as usize].read();
	let result = (*guard).is_some();
	drop(guard);
	result
}

/// Terminate a job
/// Returns true if the process exists and has been successfully terminated, false otherwise.
pub fn terminate(pid: u8) -> bool {
	use super::process::get_state;

	if !process_exists(pid) {
		return false;
	}

	let mut pt_guard = PROCESS_TABLE[pid as usize].write(); // lock process in process table
	let state = get_state(pt_guard.as_ref().unwrap());
	//((pt_guard.as_ref().unwrap().0).1).0 = State::Limbo(Limbo::Terminated); // set process state to terminated
	match state {
		State::MainMemory(MainMemory::Running) => {
			let guard = RUNNING.read();
			if (*guard).is_some() {
				if (*guard).unwrap() == pid {
					let mut wguard = RUNNING.write();
					(*wguard).take();
					drop(wguard);
				}
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
}

/*
	PROCESSES_COUNT
*/

pub fn get_process_count() -> u8 {
	let guard = PROCESS_COUNT.read();
	let value = (*guard).clone();
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

/*
	QUEUES
*/

pub fn queue_push_back(queue: &Mutex<ArrayDeque<[u8; 256]>>, pid: u8, state: State) -> Result<(), CapacityError<u8>> {
	let mut q_guard = queue.lock();

	if !process_exists(pid) || (*q_guard).contains(&pid) {
		return Ok(());
	}

	let result = (*q_guard).push_back(pid);
	if result.is_ok() {
		let mut pt_guard = PROCESS_TABLE[pid as usize].write();
		set_state(&mut (*pt_guard).unwrap(), state);
		drop(pt_guard);
	}
	drop(q_guard);

	result
}

pub fn queue_remove(queue: &Mutex<ArrayDeque<[u8; 256]>>, pid: u8) -> bool {
	let mut guard = queue.lock();

	for index in 0..(*guard).len() {
		if (*guard)[index] == pid {
			(*guard).remove(index);
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
	let value = (*guard).len();
	drop(guard);
	value
}
