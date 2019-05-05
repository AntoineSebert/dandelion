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
	use super::{get_process_count, queue_size, swapper::get_running};

	(get_process_count(), queue_size(&BLOCKED_QUEUE), queue_size(&READY_QUEUE), get_running())
}

/// Remove the processes those deadlines have been missed in the given queue.
fn terminator(queue: &Mutex<ArrayDeque<[u8; 256]>>) -> u8 {
	use crate::kernel::{scheduler::PROCESS_TABLE, time::get_datetime};
	use core::cmp::Ordering::Less;

	let mut guard = queue.lock();
	let mut counter = 0;
	for index in 0..(*guard).len() {
		let pt_guard = PROCESS_TABLE[index as usize].read();
		let realtime = (*pt_guard).as_ref().unwrap().get_periodicity();
		if realtime.is_some() && realtime.unwrap().is_right() {
			let deadline = realtime.unwrap().right().unwrap().1;
			if deadline.cmp(&get_datetime()) == Less {
				(*guard).remove(index);
				counter += 1;
			}
		}
		drop(pt_guard);
	}
	drop(guard);
	counter
}

pub mod strategy {
	use crate::kernel::scheduler::PROCESS_TABLE;
	use arraydeque::ArrayDeque;
	use core::cmp::Ordering::*;
	use either::{Left, Right};
	use spin::Mutex;

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
			let pt_guard = PROCESS_TABLE[*a as usize].read();
			let priority_a = (*pt_guard).as_ref().unwrap().get_priority();
			drop(pt_guard);
			let pt_guard = PROCESS_TABLE[*b as usize].read();
			let priority_b = (*pt_guard).as_ref().unwrap().get_priority();
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
			use crate::kernel::process::PRIORITY::*;

			let pt_guard_a = PROCESS_TABLE[*a as usize].read();
			let process = (*pt_guard_a).as_ref().unwrap();
			let constraint_a = process.get_constraint();
			let pt_guard_b = PROCESS_TABLE[*b as usize].read();
			let process = (*pt_guard_b).as_ref().unwrap();
			let constraint_b = process.get_constraint();

			let order = match (constraint_a.0, constraint_b.0) {
				(None, None) => {
					match constraint_a.1.partial_cmp(&constraint_b.1) {
						Some(ord) => ord,
						None => Equal,
					}
				}
				(None, Some(_)) => Less,
				(Some(_), None) => Greater,
				(Some(periodicity_a), Some(periodicity_b)) => {
					match (periodicity_a, periodicity_b) {
						(Right(aperiodic_a), Right(aperiodic_b)) => {
							match aperiodic_a.1.partial_cmp(&aperiodic_b.1) {
								Some(ord) => {
									match (constraint_a.1, constraint_b.1) {
										(HIGH, MEDIUM) => Greater,
										(MEDIUM, LOW) => Greater,
										(LOW, MEDIUM) => Less,
										(MEDIUM, HIGH) => Less,
										_ => ord,
									}
								}
								None => {
									match constraint_a.1.partial_cmp(&constraint_b.1) {
										Some(ord) => ord,
										None => Equal,
									}
								}
							}
						}
						(Right(aperiodic_a), Left(periodic_b)) => {
							match aperiodic_a.0.partial_cmp(&periodic_b.1) {
								Some(ord) => {
									match (constraint_a.1, constraint_b.1) {
										(HIGH, MEDIUM) => Greater,
										(MEDIUM, LOW) => Greater,
										(LOW, MEDIUM) => Less,
										(MEDIUM, HIGH) => Less,
										_ => ord,
									}
								}
								None => {
									match constraint_a.1.partial_cmp(&constraint_b.1) {
										Some(ord) => ord,
										None => Equal,
									}
								}
							}
						}
						(Left(periodic_a), Right(aperiodic_b)) => {
							match periodic_a.1.partial_cmp(&aperiodic_b.0) {
								Some(ord) => {
									match (constraint_a.1, constraint_b.1) {
										(HIGH, MEDIUM) => Greater,
										(MEDIUM, LOW) => Greater,
										(LOW, MEDIUM) => Less,
										(MEDIUM, HIGH) => Less,
										_ => ord,
									}
								}
								None => {
									match constraint_a.1.partial_cmp(&constraint_b.1) {
										Some(ord) => ord,
										None => Equal,
									}
								}
							}
						}
						(Left(periodic_a), Left(periodic_b)) => {
							match periodic_a.2.partial_cmp(&periodic_b.2) {
								Some(ord) => {
									match (constraint_a.1, constraint_b.1) {
										(HIGH, MEDIUM) => Greater,
										(MEDIUM, LOW) => Greater,
										(LOW, MEDIUM) => Less,
										(MEDIUM, HIGH) => Less,
										_ => ord,
									}
								}
								None => {
									match constraint_a.1.partial_cmp(&constraint_b.1) {
										Some(ord) => ord,
										None => Equal,
									}
								}
							}
						}
					}
				}
			};
			drop(pt_guard_a);
			drop(pt_guard_b);

			order
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
			let pt_guard_a = PROCESS_TABLE[*a as usize].read();
			let periodicity_a = (*pt_guard_a).as_ref().unwrap().get_periodicity();

			let pt_guard_b = PROCESS_TABLE[*b as usize].read();
			let periodicity_b = (*pt_guard_b).as_ref().unwrap().get_periodicity();

			let order = match (periodicity_a, periodicity_b) {
				(None, None) => Equal,
				(None, Some(_)) => Less,
				(Some(_), None) => Greater,
				(Some(periodicity_a), Some(periodicity_b)) => {
					match (periodicity_a, periodicity_b) {
						(Right(aperiodic_a), Right(aperiodic_b)) => {
							match aperiodic_a.1.partial_cmp(&aperiodic_b.1) {
								Some(ord) => ord,
								None => Equal,
							}
						}
						(Right(aperiodic_a), Left(periodic_b)) => {
							match aperiodic_a.0.partial_cmp(&periodic_b.1) {
								Some(ord) => ord,
								None => Equal,
							}
						}
						(Left(periodic_a), Right(aperiodic_b)) => {
							match periodic_a.1.partial_cmp(&aperiodic_b.0) {
								Some(ord) => ord,
								None => Equal,
							}
						}
						(Left(periodic_a), Left(periodic_b)) => {
							match periodic_a.2.partial_cmp(&periodic_b.2) {
								Some(ord) => ord,
								None => Equal,
							}
						}
					}
				}
			};
			drop(pt_guard_a);
			drop(pt_guard_b);

			order
		});
		let first_value = Some(*(*guard).front().unwrap());

		drop(guard);
		first_value
	}
}
