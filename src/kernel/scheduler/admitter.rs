/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

use super::super::{process::Main, PROCESS_TABLE};

/// Check whether the task can be accepted or not
/// If yes, a process is constructed and add to the process queue & job table, and true is returned
/// Otherwise, returns false.
pub fn request(candidate: Main) -> bool {
	let slot = get_slot();
	if slot.is_err() {
		return false;
	}
	if !is_schedulable(candidate) {
		return false;
	}

	// create a process and add it to process table & process queue

	return true;
}

/// Browse PROCESS_TABLE. The outcome of the operation is returned as a Result.
fn get_slot() -> Result<u8, &'static str> {
	for index in 0..256 {
		/*
		if PROCESS_TABLE[index].read().is_none() {
			return Ok(index as u8);
		}
		*/
	}
	Err("No slot available for a new process")
}

/// Figure out if the candidate is schedulable in the current context.
fn is_schedulable(candidate: Main) -> bool {
	//

	false
}

/// Creates a new process and add it ot the PROCESS_TABLE, and stores its index in PROCESS_QUEUE.
fn admit<F>(candidate: Main) {}
