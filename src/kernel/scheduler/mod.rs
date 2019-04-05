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

use spin::RwLock;
use array_init::array_init;
use super::process::*;
use arraydeque::ArrayDeque;
use core::{ptr::null_mut, sync::atomic::{AtomicPtr, self}};
use lazy_static::lazy_static;
use spin::Mutex;

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
	pub static ref RUNNING: AtomicPtr<u8> = AtomicPtr::new(null_mut());
}

pub fn process_exists(pid: u8) -> bool {
	let guard = PROCESS_TABLE[pid as usize].read();
	let result = (*guard).is_some();
	drop(guard);
	result
}

/// Return the number of processes in PROCESS_TABLE
/// Does not guaranty accuracy :
/// For performance and concurrency reasons, PROCESS_TABLE cannot be locked as a whole
/// Processes may be added or removed during the counting process.
pub fn count_processes() -> u8 {
	let mut counter = 0;
	for element in PROCESS_TABLE.iter() {
		let guard = element.read();
		if (*guard).is_some() {
			counter += 1;
		}
		drop(guard);
	}
	counter
}

/// Get the process State, assuming the process exists
pub fn get_state(pid: usize) -> State {
	let guard = PROCESS_TABLE[pid].read();
	let state = ((&(*guard).as_ref().unwrap().0).1).0; // create a macro to hide this shit
	drop(guard);
	state
}

pub fn remove_pid_from_queue(mut queue: spin::MutexGuard<ArrayDeque<[u8; 256]>>, pid: u8) -> bool {
	let mut index = None;
	for element in queue.iter() {
		if *element == pid {
			index = Some(*element);
			break;
		}
	}
	if index.is_some() {
		queue.remove(index.unwrap() as usize);
	}
	index.is_some()
}

/// Terminate a job
/// Returns true if the process exists and has been successfully terminated, false otherwise.
pub fn terminate(pid: u8) -> bool {
	use atomic::Ordering::Relaxed;

	if !process_exists(pid) {
		return false;
	}

	let mut pt_guard = PROCESS_TABLE[pid as usize].write(); // lock process in process table
	// set process state to terminated
	let state = ((&(*pt_guard).as_ref().unwrap().0).1).0;
	match state {
		State::MainMemory(MainMemory::Running) => { RUNNING.compare_exchange(pid as *mut _, null_mut(), Relaxed, Relaxed).ok(); },
		State::MainMemory(MainMemory::Ready) => { remove_pid_from_queue(READY_QUEUE.lock(), pid); },
		State::SwapSpace(_) => { remove_pid_from_queue(BLOCKED_QUEUE.lock(), pid); },
		_ => {}
	};
	*pt_guard = None;
	drop(pt_guard);

	true
}
