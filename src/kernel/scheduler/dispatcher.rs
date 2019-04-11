/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */
/*
use crate::kernel::scheduler::RUNNING;
use crate::kernel::scheduler::BLOCKED_QUEUE;
use crate::kernel::scheduler::READY_QUEUE;
*/
/// Update the scheduling process.
/// Return a tuple containing :
/// - the total number of processes
/// - the number of processes in BLOCKED_QUEUE
/// - the number of processes in READY_QUEUE
/// - the PID of eventual running process
pub fn update() -> (u8, usize, usize, Option<u8>) {
	// do the scheduling
	/*
	let bq_size = BLOCKED_QUEUE.lock().len();
	let rq_size = READY_QUEUE.lock().len();
	*/
	(/*super::count_processes()*/0, /*bq_size*/0, /*rq_size*/0, Some(0))
}
