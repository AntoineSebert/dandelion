/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

use super::{super::process::*, PROCESS_TABLE};

/// Check whether the task can be accepted or not
/// If yes, a process is constructed and add to the process queue & job table, and true is returned
/// Otherwise, returns false.
pub fn request(constraint: Constraint, code: Runnable) -> Option<usize> {
	let slot = get_slot();

	if slot.is_none() || !is_schedulable(constraint) {
		return None;
	}

	admit(constraint, code, slot.unwrap());
	slot
}

/// Browse PROCESS_TABLE and return the first available slot if it exists.
fn get_slot() -> Option<usize> {
	for index in 0..256 {
		let guard = PROCESS_TABLE[index].read();
		if (*guard).is_none() {
			return Some(index as usize);
		}
		drop(guard);
	}
	None
}

/// Figure out if the candidate is schedulable in the current context.
fn is_schedulable(constraint: Constraint) -> bool {
	// https://fr.wikipedia.org/wiki/Rate-monotonic_scheduling
	if constraint.0.is_none() {
		true
	} else {
		use strategy::*;

		rate_monotonic(constraint)
	}
}

/// Creates a new process and add it ot the PROCESS_TABLE, and stores its index in PROCESS_QUEUE.
fn admit(constraint: Constraint, code: Runnable, index: usize) {
	use super::super::time;
	use crate::println;
	use core::time::Duration;

	let create_task = |constraint: Constraint, code: Runnable| -> Task {
		let create_metadata = |constraint: Constraint| -> Metadata {
			(constraint, (State::Limbo(Limbo::Creating), <Duration>::new(0, 0), time::get_datetime()))
		};

		(create_metadata(constraint), code)
	};

	let mut guard = PROCESS_TABLE[index].write();
	*guard = Some(create_task(constraint, code));
	drop(guard);

	super::increment();

	println!("New process admitted at index {}", index);
}

pub mod strategy {
	use crate::kernel::{
		process::{get_estimated_remaining_time, get_realtime, Constraint, Task},
		scheduler::PROCESS_TABLE,
	};
	use arraydeque::ArrayDeque;
	use either::Either::{Left, Right};
	use num_traits::pow::pow;
	use spin::RwLockWriteGuard;

	pub fn rate_monotonic(constraint: Constraint) -> bool {
		let realtime_tasks: ArrayDeque<[RwLockWriteGuard<Option<Task>>; 256]> = {
			let mut temp: ArrayDeque<_> = ArrayDeque::new();
			for element in PROCESS_TABLE.iter() {
				let guard = element.read();
				if let Some(v) = *guard {
					if get_realtime(&v).is_some() {
						// capacity error should never happen if PROCESS_TABLE and realtime_tasks have the same size
						if let Ok(()) = temp.push_back(element.write()) {}
					}
				}
				drop(guard);
			}
			temp
		};

		let rate: f64 = {
			let mut temp = 0.0;
			for task in realtime_tasks.iter() {
				temp += match get_realtime(&(task).unwrap()).unwrap() {
					Left(periodic) => periodic.0.as_secs() as f64 / periodic.1.as_secs() as f64,
					Right(_) => get_estimated_remaining_time(&(task).unwrap()).as_secs() as f64 / 256_f64,
				}
			}

			temp += match constraint.0.unwrap() {
				Left(periodic) => periodic.0.as_secs() as f64 / periodic.1.as_secs() as f64,
				Right(aperiodic) => aperiodic.0.as_secs() as f64 / 256_f64,
			};

			temp
		};

		let n = realtime_tasks.len();

		for guard in realtime_tasks {
			drop(guard);
		}

		rate < (n as f64) * (pow(2.0, 1 / n) - 1.0)
	}
}
