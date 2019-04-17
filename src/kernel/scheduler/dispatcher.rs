/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

use spin::Mutex;
use arraydeque::ArrayDeque;
use super::{BLOCKED_QUEUE, queue_size, READY_QUEUE};

/// Reorder READY_QUEUE with the provided strategy if READY_QUEUE is not empty.
/// Return the data collected by `global_info()`.
pub fn update(strategy: &Fn(&Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8>) -> (u8, usize, usize, Option<u8>) {
	if 0 < queue_size(&READY_QUEUE) {
		strategy(&READY_QUEUE);
	}

	global_info()
}

/// Get scheduler's components info.
/// Return a tuple containing :
/// - the total number of processes
/// - the number of processes in BLOCKED_QUEUE
/// - the number of processes in READY_QUEUE
/// - the PID of eventual running process
pub fn global_info() -> (u8, usize, usize, Option<u8>) {
	use super::{get_process_count, queue_size, swapper::get_running};

	(get_process_count(), queue_size(&BLOCKED_QUEUE), queue_size(&READY_QUEUE), get_running())
}

pub mod strategy {
	use crate::kernel::{process::get_priority, scheduler::PROCESS_TABLE};
	use spin::Mutex;
	use arraydeque::ArrayDeque;

	/// Put the processes in READY_QUEUE by order of PID.
	/// Return the PID of the first process in the ready queue if it exists.
	pub fn process_id(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8> {
		let mut guard = queue.lock();

		((*guard).as_mut_slices().0).sort_unstable();
		let first_value = Some(*(*guard).front().unwrap());

		drop(guard);
		first_value
	}

	/// Put the processes in READY_QUEUE by order of priority.
	/// Return the PID of the first process in the ready queue if it exists.
	pub fn priority(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8> {
		let mut guard = queue.lock();

		((*guard).as_mut_slices().0).sort_unstable_by(|a, b| {
			let guard = PROCESS_TABLE[*a as usize].read();
			let priority_a = get_priority(&(*guard).unwrap());
			drop(guard);
			let guard = PROCESS_TABLE[*b as usize].read();
			let priority_b = get_priority(&(*guard).unwrap());
			drop(guard);

			priority_a.partial_cmp(&priority_b).unwrap()
		});
		let first_value = Some(*(*guard).front().unwrap());

		drop(guard);
		first_value
	}

	/// Put the processes in READY_QUEUE by order of deadline and priority.
	/// LOW can preempt MEDIUM, MEDIUM can preempt HIGH.
	/// Return the PID of the first process in the ready queue if it exists.
	pub fn modified_earliest_deadline_first(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8> {
		let mut guard = queue.lock();

		((*guard).as_mut_slices().0).sort_unstable_by(|a, b| {
			// todo
			core::cmp::Ordering::Equal
		});
		let first_value = Some(*(*guard).front().unwrap());

		drop(guard);
		first_value
	}

	/// Put the processes in READY_QUEUE by order of deadline and priority.
	/// Return the PID of the first process in the ready queue if it exists.
	pub fn earliest_deadline_first(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8> {
		let mut guard = queue.lock();

		((*guard).as_mut_slices().0).sort_unstable_by(|a, b| {
			// todo
			core::cmp::Ordering::Equal
		});
		let first_value = Some(*(*guard).front().unwrap());

		drop(guard);
		first_value
	}
}
