/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

pub mod ipc;
pub mod scheduler;
pub mod shell;
pub mod vmm;

pub mod process;
pub mod time;

use crate::kernel::process::*;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
	pub static ref CMOS: Mutex<cmos::CMOS> = { Mutex::new(unsafe { cmos::CMOS::new() }) };
}

// should be replaced by a set
lazy_static! {
	pub static ref PROCESS_TABLE: Mutex<[Task; 256]> = { Mutex::new([Task::default(); 256]) };
}
