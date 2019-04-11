/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

#![allow(dead_code)]

use cmos::RTCDateTime;
use core::time::Duration;
use either::Either;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
	Limbo(Limbo),
	MainMemory(MainMemory),
	SwapSpace(SwapSpace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Limbo {
	Creating,
	Killed,
	Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMemory {
	Ready,
	Running,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapSpace {
	Interrupted,
	Suspended,
	Delayed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PRIORITY {
	HIGH,
	MEDIUM,
	LOW
}

pub type Arguments<'a> = [Option<&'a str>; 256];

pub type Periodic = (Duration, Duration, Option<RTCDateTime>); // estimated completion time, interval, delay
pub type Aperiodic = (Duration, RTCDateTime, Option<RTCDateTime>); // estimated completion time, deadline, delay

pub type Info = (State, Duration, RTCDateTime);
pub type Constraint = (Option<Either<Periodic, Aperiodic>>, PRIORITY);

pub type Metadata = (Constraint, Info);

pub type Runnable = fn(Arguments) -> u64;
pub type Task = (Metadata, Runnable);

pub type Job = (Metadata, [Runnable; 256]);
pub type Group = [Task; 256];

/// Accessors

pub fn get_metadata(task: &Task) -> &Metadata {
	&task.0
}

pub fn get_constraint(task: &Task) -> &Constraint {
	&(task.0).0
}

pub fn get_realtime(task: &Task) -> Option<Either<Periodic, Aperiodic>> {
	((task.0).0).0
}

pub fn get_runnable(task: &Task) -> &Runnable {
	&task.1
}

pub fn get_priority(task: &Task) -> PRIORITY {
	((task.0).0).1
}

pub fn get_info(task: &Task) -> &Info {
	&(task.0).1
}

pub fn get_state(task: &Task) -> State {
	((task.0).1).0
}

pub fn get_running_time(task: &Task) -> Duration {
	((task.0).1).1
}

pub fn get_creation_time(task: &Task) -> RTCDateTime {
	((task.0).1).2
}

/// Sample processes

pub fn sample_runnable_2(args: Arguments) -> u64 {
	use crate::println;

	println!("Running sample_runnable_2");
	for element in args.iter() {
		if element.is_some() {
			println!("argument: {}", element.unwrap());
		} else {
			break;
		}
	}

	1
}

/// Sample job streaming prime numbers on the serial port up to a limit (passed as parameter) less than 2^64
/// On my computer, find all the primes between 0 and 1.000.000 in 2:05 min
#[allow(dead_code)]
fn sample_runnable(limit: u64, output: bool /* args: Arguments */) {
	use crate::{println, serial_println};
	use integer_sqrt::IntegerSquareRoot;

	// arg 0 is name
	// arg 1 is --limit=number
	// arg 2 is output=true/false

	if output {
		println!("2");
	} else {
		serial_println!("2");
	}
	let mut candidate: u64 = 3;
	loop {
		if limit < candidate {
			break;
		}
		let mut iterator = 3;
		let mut is_prime = true;
		loop {
			if candidate.integer_sqrt() < iterator {
				break;
			}
			if candidate % iterator == 0 {
				is_prime = false;
				break;
			}
			iterator += 2;
		}
		if is_prime {
			if output {
				println!("{}", candidate);
			} else {
				serial_println!("{}", candidate);
			}
		}
		candidate += 2;
	}
}
