/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

// https://wiki.osdev.org/Loading_a_Process
// https://wiki.osdev.org/Multiprocessor_Scheduling
// https://wiki.osdev.org/Scheduling_Algorithms

pub mod admitter;
pub mod dispatcher;
pub mod swapper;

// process queue

use spin::Mutex;
use lazy_static::lazy_static;
use arraydeque::ArrayDeque;
use super::process::Task;

lazy_static! {
	pub static ref READY_QUEUE: Mutex<ArrayDeque<[Task; 256]>> = Mutex::new(ArrayDeque::new());
}

lazy_static! {
	pub static ref RUNNING: Mutex<Task> = Mutex::new(Task::default());
}

lazy_static! {
	pub static ref BLOCKED_QUEUE: Mutex<ArrayDeque<[Task; 256]>> = Mutex::new(ArrayDeque::new());
}