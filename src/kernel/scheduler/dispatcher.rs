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
	use crate::kernel::{process::{get_priority, get_realtime, get_constraint}, scheduler::PROCESS_TABLE};
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
			use core::cmp::Ordering::Equal;

			let pt_guard = PROCESS_TABLE[*a as usize].read();
			let priority_a = get_priority(&(*pt_guard).unwrap());
			drop(pt_guard);
			let pt_guard = PROCESS_TABLE[*b as usize].read();
			let priority_b = get_priority(&(*pt_guard).unwrap());
			drop(pt_guard);

			match priority_a.partial_cmp(&priority_b) {
				Some(ord) => ord,
				None => Equal,
			}
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
			use core::cmp::Ordering::*;
			use either::{Right, Left};
			use crate::kernel::process::PRIORITY::*;

			let pt_guard = PROCESS_TABLE[*a as usize].read();
			let process = (*pt_guard).unwrap();
			let constraint_a = get_constraint(&process);
			drop(pt_guard);
			let pt_guard = PROCESS_TABLE[*b as usize].read();
			let process = (*pt_guard).unwrap();
			let constraint_b = get_constraint(&process);
			drop(pt_guard);

			match (constraint_a.0, constraint_b.0) {
				(None, None) => match constraint_a.1.partial_cmp(&constraint_b.1) {
					Some(ord) => ord,
					None => Equal,
				},
				(None, Some(_)) => Less,
				(Some(_), None) => Greater,
				(Some(periodicity_a), Some(periodicity_b)) => match (periodicity_a, periodicity_b) {
					(Right(aperiodic_a), Right(aperiodic_b)) => match aperiodic_a.1.partial_cmp(&aperiodic_b.1) {
						Some(ord) => match (constraint_a.1, constraint_b.1) {
							(HIGH, MEDIUM) => Greater,
							(MEDIUM, LOW) => Greater,
							(LOW, MEDIUM) => Less,
							(MEDIUM, HIGH) => Less,
							_ => ord,
						},
						None => match constraint_a.1.partial_cmp(&constraint_b.1) {
							Some(ord) => ord,
							None => Equal,
						},
					},
					(Right(aperiodic_a), Left(periodic_b)) => match aperiodic_a.0.partial_cmp(&periodic_b.1) {
						Some(ord) => match (constraint_a.1, constraint_b.1) {
							(HIGH, MEDIUM) => Greater,
							(MEDIUM, LOW) => Greater,
							(LOW, MEDIUM) => Less,
							(MEDIUM, HIGH) => Less,
							_ => ord,
						},
						None => match constraint_a.1.partial_cmp(&constraint_b.1) {
							Some(ord) => ord,
							None => Equal,
						},
					},
					(Left(periodic_a), Right(aperiodic_b)) => match periodic_a.1.partial_cmp(&aperiodic_b.0) {
						Some(ord) => match (constraint_a.1, constraint_b.1) {
							(HIGH, MEDIUM) => Greater,
							(MEDIUM, LOW) => Greater,
							(LOW, MEDIUM) => Less,
							(MEDIUM, HIGH) => Less,
							_ => ord,
						},
						None => match constraint_a.1.partial_cmp(&constraint_b.1) {
							Some(ord) => ord,
							None => Equal,
						},
					},
					(Left(periodic_a), Left(periodic_b)) => match periodic_a.2.partial_cmp(&periodic_b.2) {
						Some(ord) => match (constraint_a.1, constraint_b.1) {
							(HIGH, MEDIUM) => Greater,
							(MEDIUM, LOW) => Greater,
							(LOW, MEDIUM) => Less,
							(MEDIUM, HIGH) => Less,
							_ => ord,
						},
						None => match constraint_a.1.partial_cmp(&constraint_b.1) {
							Some(ord) => ord,
							None => Equal,
						},
					},
				},
			}
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
			use core::cmp::Ordering::*;
			use either::{Right, Left};

			let pt_guard = PROCESS_TABLE[*a as usize].read();
			let realtime_a = get_realtime(&(*pt_guard).unwrap());
			drop(pt_guard);
			let pt_guard = PROCESS_TABLE[*b as usize].read();
			let realtime_b = get_realtime(&(*pt_guard).unwrap());
			drop(pt_guard);

			match (realtime_a, realtime_b) {
				(None, None) => Equal,
				(None, Some(_)) => Less,
				(Some(_), None) => Greater,
				(Some(periodicity_a), Some(periodicity_b)) => match (periodicity_a, periodicity_b) {
					(Right(aperiodic_a), Right(aperiodic_b)) => match aperiodic_a.1.partial_cmp(&aperiodic_b.1) {
						Some(ord) => ord,
						None => Equal,
					},
					(Right(aperiodic_a), Left(periodic_b)) => match aperiodic_a.0.partial_cmp(&periodic_b.1) {
						Some(ord) => ord,
						None => Equal,
					},
					(Left(periodic_a), Right(aperiodic_b)) => match periodic_a.1.partial_cmp(&aperiodic_b.0) {
						Some(ord) => ord,
						None => Equal,
					},
					(Left(periodic_a), Left(periodic_b)) => match periodic_a.2.partial_cmp(&periodic_b.2) {
						Some(ord) => ord,
						None => Equal,
					},
				},
			}
		});
		let first_value = Some(*(*guard).front().unwrap());

		drop(guard);
		first_value
	}
}
