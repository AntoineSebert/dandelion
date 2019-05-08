use super::{queue_size, BLOCKED_QUEUE, READY_QUEUE};
use arraydeque::ArrayDeque;
use spin::Mutex;

/// Reorder READY_QUEUE with the provided strategy if READY_QUEUE is not empty.
/// Return the data collected by `global_info()`.
pub fn update(strategy: &Fn(&Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8>) -> (u8, usize, usize, Option<u8>) {
	if 0 < queue_size(&READY_QUEUE) {
		terminator(&READY_QUEUE);
		strategy(&READY_QUEUE);
	}

	if 0 < queue_size(&BLOCKED_QUEUE) {
		terminator(&BLOCKED_QUEUE);
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
	use super::{get_process_count, swapper::get_running};

	(get_process_count(), queue_size(&BLOCKED_QUEUE), queue_size(&READY_QUEUE), get_running())
}

/// Remove the processes those deadlines have been missed in the given queue.
fn terminator(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> u8 {
	use crate::kernel::{
		scheduler::PROCESS_TABLE,
		time::{dt_add_du, get_datetime},
	};
	use either::{Left, Right};

	let mut guard = queue.lock();
	let mut counter = 0;
	for index in 0..guard.len() {
		if let Some(periodicity) = PROCESS_TABLE[index as usize].read().as_ref().unwrap().get_periodicity() {
			match periodicity {
				Left(periodic) => {
					if dt_add_du(periodic.2, periodic.1).unwrap() < get_datetime() {
						guard.remove(index);
						counter += 1;
					}
				}
				Right(aperiodic) => {
					if aperiodic.1 < get_datetime() {
						guard.remove(index);
						counter += 1;
					}
				}
			}
		}
	}

	counter
}

pub mod strategy {
	use crate::kernel::{
		process::ord_periodicity,
		scheduler::{queue_front, PROCESS_TABLE},
	};
	use arraydeque::ArrayDeque;
	use core::cmp::Ordering::*;
	use spin::Mutex;

	/// Put the processes in READY_QUEUE by order of PID.
	/// Return the PID of the first process in the ready queue if it exists.
	pub fn process_id(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8> {
		queue.lock().as_mut_slices().0.sort_unstable();
		queue_front(queue)
	}

	/// Put the processes in READY_QUEUE by order of priority.
	/// Return the PID of the first process in the ready queue if it exists.
	pub fn priority(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8> {
		queue.lock().as_mut_slices().0.sort_unstable_by(|a, b| {
			PROCESS_TABLE[*a as usize]
				.read()
				.as_ref()
				.unwrap()
				.get_priority()
				.cmp(&PROCESS_TABLE[*b as usize].read().as_ref().unwrap().get_priority())
		});

		queue_front(queue)
	}

	/// Put the processes in READY_QUEUE by order of deadline and priority.
	/// LOW can preempt MEDIUM, MEDIUM can preempt HIGH.
	/// Return the PID of the first process in the ready queue if it exists.
	pub fn modified_earliest_deadline_first(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8> {
		queue.lock().as_mut_slices().0.sort_unstable_by(|a, b| {
			use crate::kernel::process::PRIORITY::*;

			let guard_a = PROCESS_TABLE[*a as usize].read();
			let guard_b = PROCESS_TABLE[*b as usize].read();

			let process_a = guard_a.as_ref().unwrap();
			let process_b = guard_b.as_ref().unwrap();

			match (process_a.get_periodicity(), process_b.get_periodicity()) {
				(None, None) => process_a.get_priority().cmp(&process_b.get_priority()),
				(None, Some(_)) => Less,
				(Some(_), None) => Greater,
				(Some(periodicity_a), Some(periodicity_b)) => {
					match (process_a.get_priority(), process_b.get_priority()) {
						(HIGH, LOW) => ord_periodicity(&periodicity_a, &periodicity_b),
						(LOW, HIGH) => ord_periodicity(&periodicity_a, &periodicity_b),
						_ => process_a.get_priority().cmp(&process_b.get_priority()),
					}
				}
			}
		});

		queue_front(queue)
	}

	/// Put the processes in READY_QUEUE by order of deadline and priority.
	/// Return the PID of the first process in the ready queue if it exists.
	pub fn earliest_deadline_first(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> Option<u8> {
		queue.lock().as_mut_slices().0.sort_unstable_by(|a, b| {
			match (
				PROCESS_TABLE[*a as usize].read().as_ref().unwrap().get_periodicity(),
				PROCESS_TABLE[*b as usize].read().as_ref().unwrap().get_periodicity(),
			) {
				(None, None) => Equal,
				(None, Some(_)) => Less,
				(Some(_), None) => Greater,
				(Some(periodicity_a), Some(periodicity_b)) => ord_periodicity(periodicity_a, periodicity_b),
			}
		});

		queue_front(queue)
	}
}
