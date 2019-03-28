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

use super::process::Task;
use arraydeque::ArrayDeque;
use core::{ptr::null_mut, sync::atomic::AtomicPtr};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
	pub static ref READY_QUEUE: Mutex<ArrayDeque<[Task; 256]>> = Mutex::new(ArrayDeque::new());
}

lazy_static! {
	pub static ref RUNNING: AtomicPtr<Task> = AtomicPtr::new(null_mut());
}

lazy_static! {
	pub static ref BLOCKED_QUEUE: Mutex<ArrayDeque<[Task; 256]>> = Mutex::new(ArrayDeque::new());
}
