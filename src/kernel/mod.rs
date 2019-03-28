/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

// https://wiki.osdev.org/ACPI
// https://wiki.osdev.org/Detecting_CPU_Speed
// https://wiki.osdev.org/Creating_an_Operating_System#Multithreaded_Kernel

pub mod ipc;
pub mod scheduler;
pub mod shell;
pub mod vmm;

pub mod process;
pub mod time;
pub mod acpi;
pub mod interrupts;

use crate::kernel::process::*;
use array_init::array_init;
use core::option::Option;
use lazy_static::lazy_static;
use spin::{Mutex, RwLock};

// CMOS
lazy_static! {
	pub static ref CMOS: Mutex<cmos::CMOS> = { Mutex::new(unsafe { cmos::CMOS::new() }) };
}

// should be replaced by a set
lazy_static! {
	pub static ref PROCESS_TABLE: [RwLock<Option<Task>>; 256] = { array_init(|_| RwLock::new(None)) };
}
