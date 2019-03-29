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

pub mod acpi;
pub mod interrupts;
pub mod process;
pub mod time;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
	pub static ref CMOS: Mutex<cmos::CMOS> = { Mutex::new(unsafe { cmos::CMOS::new() }) };
}
