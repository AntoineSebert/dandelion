/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

use crate::kernel::PROCESS_TABLE;

pub type Arguments = [[char; 256]; 256];
pub trait MainFn = Fn(Arguments) -> i64;

/// Check whether the task can be accepted or not
/// If yes, a process is constructed and add to the process queue & job table, and true is returned
/// Otherwise, returns false.
pub fn request<F>(candidate: F, args: Arguments) -> bool
where
	F: MainFn,
{
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
		if PROCESS_TABLE[index].read().is_none() {
			return Ok(index as u8);
		}
	}
	Err("No slot available for a new process")
}

/// Figure out if the candidate is schedulable in the current context.
fn is_schedulable<F>(candidate: F) -> bool where F: MainFn {
	//

	false
}

/// Creates a new process and add it ot the PROCESS_TABLE, and stores its index in PROCESS_QUEUE.
fn admit<F>(candidate: F, args: Arguments) where F: MainFn {

}