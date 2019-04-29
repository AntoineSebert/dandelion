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
pub mod serial;
pub mod time;
pub mod vga_buffer;
