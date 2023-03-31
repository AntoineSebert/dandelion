use super::super::process::{Constraint, Runnable};

/// Check whether the task can be accepted or not.
/// If yes, a process is constructed and add to the process queue & job table, and true is returned.
/// Otherwise, returns false.
#[must_use]
pub fn request(constraint: Constraint, code: Runnable) -> Option<usize> {
	use super::{add_task, get_slot};

	if is_schedulable(constraint) {
		if let Some(slot) = get_slot() {
			add_task(constraint, code, slot);
			Some(slot)
		} else {
			None
		}
	} else {
		None
	}
}

/// Figure out if the candidate is schedulable in the current context.
fn is_schedulable(constraint: Constraint) -> bool {
	if constraint.0.is_none() {
		true
	} else {
		strategy::rate_monotonic(constraint)
	}
}

pub mod strategy {
	use crate::kernel::{
		process::{task::Task, Constraint},
		scheduler::PROCESS_TABLE,
	};
	use alloc::collections::VecDeque;
	use either::Either::{Left, Right};
	use num_traits::pow::pow;
	use spin::RwLockWriteGuard;

	#[must_use]
	pub fn rate_monotonic(constraint: Constraint) -> bool {
		let mut realtime_tasks =
			VecDeque::<RwLockWriteGuard<Option<Task>>>::with_capacity(super::super::MAX_TASKS as usize);

		for element in PROCESS_TABLE.iter() {
			if let Some(v) = element.read().as_ref() {
				if v.is_realtime() {
					// capacity error should never happen if PROCESS_TABLE and realtime_tasks have the same size
					realtime_tasks.push_back(element.write());
				}
			}
		}

		let rate: f64 = {
			let mut temp = 0.0;

			for task in &realtime_tasks {
				temp += match task.as_ref().unwrap().get_periodicity().unwrap() {
					Left(periodic) => periodic.0.as_secs_f64() / periodic.1.as_secs_f64(),
					Right(_) => task.as_ref().unwrap().get_estimated_remaining_time().unwrap().as_secs_f64() / 256_f64,
				}
			}

			temp += match constraint.0.unwrap() {
				Left(periodic) => periodic.0.as_secs_f64() / periodic.1.as_secs_f64(),
				Right(aperiodic) => aperiodic.0.as_secs_f64() / 256_f64,
			};

			temp
		};

		rate < (realtime_tasks.len() as f64) * (pow(2.0, 1 / realtime_tasks.len()) - 1.0)
	}
}
