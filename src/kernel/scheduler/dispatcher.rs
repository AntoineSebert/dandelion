/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

use crate::kernel::scheduler::{queue_size, get_process_count, get_running, set_running, BLOCKED_QUEUE, READY_QUEUE};

/// Update the scheduling process.
pub fn update() -> (u8, usize, usize, Option<u8>) {
	let strategy = first_encountered;
	set_running(strategy());

	global_info()
}

fn first_encountered() -> Option<u8> {
	/*
	for index in 0..256 {
		let guard = PROCESS_TABLE[index].read();
		if (*guard).is_some() {
			return Some(index as u8);
		}
		drop(guard);
	}
	None
	*/
	None
}

/// Get scheduler's components info
/// Return a tuple containing :
/// - the total number of processes
/// - the number of processes in BLOCKED_QUEUE
/// - the number of processes in READY_QUEUE
/// - the PID of eventual running process
pub fn global_info() -> (u8, usize, usize, Option<u8>) {
	(get_process_count(), queue_size(&BLOCKED_QUEUE), queue_size(&READY_QUEUE), get_running())
}