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
use arraydeque::ArrayDeque;
use core::{
	ptr::null_mut,
	sync::atomic::{self, AtomicPtr, Ordering::*},
};
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
	pub static ref RUNNING: AtomicPtr<u8> = AtomicPtr::new(null_mut());
}

/// Run the current running process
pub fn run() -> u64 {
	let guard = PROCESS_TABLE[RUNNING.load(SeqCst) as usize].read();
	let mut result = 0;
	if (*guard).is_none() {
		crate::println!("No process to run");
	}
	else {
		crate::println!("Running...");
		let mut args = [None; 256];
		args[0] = Some("sample_runnable_2");
		args[1] = Some("Hearth");
		result = (guard.as_ref().unwrap().1)(args);
	}
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

/// Return the number of processes in PROCESS_TABLE
/// Does not guaranty accuracy :
/// For performance and concurrency reasons, PROCESS_TABLE cannot be locked as a whole
/// Processes may be added or removed during the counting process.
pub fn count_processes() -> u8 {
	let mut counter = 0;
	for i in 0..255 {
		if process_exists(i) {
			counter += 1;
		}
	}
	counter
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
		State::MainMemory(MainMemory::Running) => {
			RUNNING.compare_exchange(pid as *mut _, null_mut(), Relaxed, Relaxed).ok();
		}
		State::MainMemory(MainMemory::Ready) => {
			remove_pid_from_queue(READY_QUEUE.lock(), pid);
		}
		State::SwapSpace(_) => {
			remove_pid_from_queue(BLOCKED_QUEUE.lock(), pid);
		}
		_ => {}
	};
	*pt_guard = None;
	drop(pt_guard);

	true
}
