/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

use super::{PROCESS_TABLE, super::process::*};

/// Check whether the task can be accepted or not
/// If yes, a process is constructed and add to the process queue & job table, and true is returned
/// Otherwise, returns false.
pub fn request(constraint: Constraint, code: Main) -> Option<usize> {
	let slot = get_slot();

	if slot.is_none() {
		return None;
	} else if !is_schedulable(constraint) {
		return None;
	}

	let index = slot.unwrap();
	admit(constraint, code, index);
	return Some(index);
}

/// Browse PROCESS_TABLE. The outcome of the operation is returned as a Result.
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
fn is_schedulable(_constraint: Constraint) -> bool {
	//

	true
}

/// Creates a new process and add it ot the PROCESS_TABLE, and stores its index in PROCESS_QUEUE.
fn admit(constraint: Constraint, code: Main, index: usize) {
	use super::super::time;
	use core::time::Duration;
	use crate::println;

	let create_task = | constraint: Constraint, code: Main | -> Task {
		let create_metadata = | constraint: Constraint | -> Metadata {
			(constraint, (State::Limbo(Limbo::Creating), <Duration>::new(0, 0), time::get_datetime()))
		};

		(create_metadata(constraint), code)
	};

	let mut guard = PROCESS_TABLE[index].write();
	*guard = Some(create_task(constraint, code));
	drop(guard);

	println!("New process admitted at index {}", index);
}
