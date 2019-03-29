/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

use super::{PROCESS_TABLE, super::process::*};

/// Check whether the task can be accepted or not
/// If yes, a process is constructed and add to the process queue & job table, and true is returned
/// Otherwise, returns false.
pub fn request(constraint: Constraint, code: Main) -> bool {
	let slot = get_slot();
	if slot.is_err() {
		return false;
	}
	if !is_schedulable(constraint) {
		return false;
	}

	// create a process and add it to process table & process queue
	admit(constraint, code, slot.unwrap());

	return true;
}

/// Browse PROCESS_TABLE. The outcome of the operation is returned as a Result.
fn get_slot() -> Result<usize, &'static str> {
	for index in 0..256 {
		if PROCESS_TABLE[index].read().is_none() {
			return Ok(index as usize);
		}
	}
	Err("No slot available for a new process")
}

/// Figure out if the candidate is schedulable in the current context.
fn is_schedulable(_constraint: Constraint) -> bool {
	//

	true
}

/// Creates a new process and add it ot the PROCESS_TABLE, and stores its index in PROCESS_QUEUE.
fn admit(constraint: Constraint, code: Main, slot: usize) {
	use super::super::time;
	use core::time::Duration;
	use crate::println;

	let create_task = | constraint: Constraint, code: Main | -> Task {
		let create_metadata = | constraint: Constraint | -> Metadata {
			(constraint, (State::Limbo(Limbo::Creating), <Duration>::new(0, 0), time::get_datetime()))
		};

		(create_metadata(constraint), code)
	};

	let mut guard = PROCESS_TABLE[slot].write();
	*guard = Some(create_task(constraint, code));
	println!("New process admitted");
}
