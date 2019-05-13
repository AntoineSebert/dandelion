// https://wiki.osdev.org/Context_Switching

/*
Yes, you typically want to save all general purpose registers. The reason is that you have to save all registers that the interrupted job uses, so that it can continue later. Note that you need to also save the SSE registers if you enable SSE at some point.

Implementing context switches is a bit tricky because you have to be very careful to not accidentally overwrite registers before saving them. One way to do this is to make the interrupt handler a naked function that does the following:

- push all registers to the stack
- call a function that retuns the stack pointer of the next thread from the thread list
- call a function that puts the current thread (represented by the stack pointer) into the thread list
- switch the stack by updating the rsp register
- pop the saved register values of the thread from stack

You probably want to use a different interrupt stack (like we did for the double fault exception) to avoid the race condition between step 3 and 4.

Note that this is only a high-level overview and only one way of doing it. For the blog I plan to use a different approach but I'm not sure if it works out yet.
*/

//pub fn context_switch(job) {}
//fn push_registers(registers) {}
//fn get_next_stack_pointer() {}
//fn get_current_task_pointer() {}
//fn rsp_update() {}
//fn pop_register() {}

use super::RUNNING;
use crate::kernel::process::{
	self,
	State::{self, MainMemory},
};

/// Return the value of RUNNING.
pub fn get_running() -> Option<u8> { *RUNNING.read() }

/// Move the first element in READY_QUEUE to RUNNING (if there is no element, RUNNING is `None`).
/// Move the element in RUNNING at the end of READY_QUEUE if it exists.
/// If the element cannot be placed in READY_QUEUE, it is moved to BLOCKED_QUEUE.
/// If the element cannot be placed in BLOCKED_QUEUE, terminated.
/// Return a tuple containing the old and the new running PIDs if they exist.
pub fn next() -> (Option<u8>, Option<u8>) {
	use super::{queue_push_back, terminate, BLOCKED_QUEUE, READY_QUEUE};
	use process::{MainMemory::Ready, SwapSpace::Suspended};
	use State::SwapSpace;

	let old = get_running();

	let mut guard = READY_QUEUE.lock();
	let new = guard.pop_front();
	set_running(new);
	drop(guard);

	if old.is_some() {
		let pid = old.unwrap();
		if queue_push_back(&READY_QUEUE, pid, MainMemory(Ready)).is_err()
			&& queue_push_back(&BLOCKED_QUEUE, pid, SwapSpace(Suspended)).is_err()
		{
			terminate(pid);
		}
	}

	(old, new)
}

/// Set the value of RUNNING and update the state of the related process if it exists.
fn set_running(value: Option<u8>) {
	use super::set_process_state;
	use process::MainMemory::Running;

	let mut r_guard = RUNNING.write();
	*r_guard = value;

	if let Some(pid) = value {
		set_process_state(pid, MainMemory(Running));
	}
}
