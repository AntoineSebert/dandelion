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
pub fn request<F>(function: F, args: Arguments) -> bool where F: MainFn {
	let slot = get_slot();
	if slot.is_none() {
		return false;
	}
	// if not schedulable return false

	// create a process and add it to process table & process queue

	return true;
}
/// Browse PROCESS_TABLE. When an empty slot is found, its index is returned in an Option.
/// If no empty slot exists, an empty Option is returned
fn get_slot() -> Option<u8> {
	for index in 0..256 {
		if PROCESS_TABLE[index].read().is_none() {
			return Some(index as u8);
		}
	}
	None
}