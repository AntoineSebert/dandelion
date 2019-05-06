use super::super::process::{Constraint, Runnable};

/// Check whether the task can be accepted or not.
/// If yes, a process is constructed and add to the process queue & job table, and true is returned.
/// Otherwise, returns false.
pub fn request(constraint: Constraint, code: Runnable) -> Option<usize> {
	use super::{add_task, get_slot};

	let slot = get_slot();

	if slot.is_none() || !is_schedulable(constraint) {
		None
	} else {
		add_task(constraint, code, slot.unwrap());
		slot
	}
}

/// Figure out if the candidate is schedulable in the current context.
fn is_schedulable(constraint: Constraint) -> bool {
	if constraint.0.is_none() {
		true
	} else {
		use strategy::*;

		rate_monotonic(constraint)
	}
}

pub mod strategy {
	use crate::kernel::{
		process::{Constraint, Task},
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
				if let Some(v) = guard.as_ref() {
					if v.is_realtime() {
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
				temp += match task.as_ref().unwrap().get_periodicity().unwrap() {
					Left(periodic) => periodic.0.as_secs_f64() / periodic.1.as_secs_f64(),
					Right(_) => task.as_ref().unwrap().get_estimated_remaining_time().unwrap().as_secs_f64() / 256_f64
				}
			}

			temp += match constraint.0.unwrap() {
				Left(periodic) => periodic.0.as_secs_f64() / periodic.1.as_secs_f64(),
				Right(aperiodic) => aperiodic.0.as_secs_f64() / 256_f64,
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
