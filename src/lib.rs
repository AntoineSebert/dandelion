#![no_std]
#![cfg_attr(test, no_main)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(custom_test_frameworks)]
#![feature(duration_float)]
#![feature(trait_alias)]

pub mod kernel;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::instructions;

#[cfg(test)]
entry_point!(test_kernel_main); // OS entry point override for tests.

/// Initialize the ACPI, the GDT,the IDT and the PICS.
/// Enables the interrupts and changes RTC interrupt rate.
pub fn init() {
	use interrupts::{change_rtc_interrupt_rate, enable_rtc_interrupt};
	use kernel::{acpi, interrupts, vmm::gdt};

	unsafe {
		match acpi::init() {
			Ok(_) => println!("ACPI initialized"),
			Err(_) => println!("Could not initialize ACPI"),
		}
	};

	gdt::init();
	interrupts::init_idt();
	unsafe { interrupts::PICS.lock().initialize() };
	instructions::interrupts::enable();

	enable_rtc_interrupt();
	change_rtc_interrupt_rate(15);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed = 0x11,
}

/// safely exit the emulator.
pub fn exit_qemu(exit_code: QemuExitCode) {
	use instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}
}

/// Halts the CPU until an interrupt occurs.
/// The program remains idle.
pub fn hlt_loop() -> ! {
	// Brøther may I have some lööps ?
	loop {
		instructions::hlt();
	}
}

/// Runs all the tests.
pub fn test_runner(tests: &[&dyn Fn()]) {
	serial_println!("Running {} tests", tests.len());
	for test in tests {
		test();
	}
	exit_qemu(QemuExitCode::Success);
}

/// Panic handler for tests that exit qemu with a `Failed` exit code.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
	serial_println!("[failed]\n");
	serial_println!("Error: {}\n", info);
	exit_qemu(QemuExitCode::Failed);
	hlt_loop();
}

/// Entry point for `cargo xtest`.
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
	init();
	test_main();
	hlt_loop();
}

/// Calls `test_panic_handler(&PanicInfo)` in case of kernel panic.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { test_panic_handler(info) }
