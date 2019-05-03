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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum PRIORITY {
	HIGH,
	MEDIUM,
	LOW,
}

// type aliases

pub type Arguments<'a> = &'a [&'a str];

/// 0 : estimated completion time
/// 1 : interval
/// 2 : last execution (can be in future for delayed tasks)
pub type Periodic = (Duration, Duration, RTCDateTime);
/// 0 : estimated completion time
/// 1 : deadline
/// 2 : start delay
pub type Aperiodic = (Duration, RTCDateTime, Option<RTCDateTime>);

pub type Info = (State, Duration, RTCDateTime);
pub type Constraint = (Option<Either<Periodic, Aperiodic>>, PRIORITY);

pub type Metadata = (Constraint, Info);

pub type Runnable = fn(Arguments) -> u64;
pub type Task = (Metadata, Runnable);

pub type Job<'a> = (Metadata, &'a [&'a Runnable]);
pub type Group<'a> = &'a [&'a Task];

// Accessors

/// Return the `Metadata` of a `Task`.
pub fn get_metadata(task: &Task) -> &Metadata { &task.0 }

pub fn get_constraint(task: &Task) -> &Constraint { &(task.0).0 }

pub fn get_realtime(task: &Task) -> Option<Either<Periodic, Aperiodic>> { ((task.0).0).0 }

pub fn get_runnable(task: &Task) -> &Runnable { &task.1 }

pub fn get_priority(task: &Task) -> PRIORITY { ((task.0).0).1 }

pub fn get_info(task: &Task) -> &Info { &(task.0).1 }

pub fn get_state(task: &Task) -> State { ((task.0).1).0 }

pub fn get_running_time(task: &Task) -> Duration { ((task.0).1).1 }

pub fn get_creation_time(task: &Task) -> RTCDateTime { ((task.0).1).2 }

pub fn get_estimated_remaining_time(task: &Task) -> Duration {
	use Either::{Left, Right};

	match get_realtime(task) {
		Some(periodicity) => {
			match periodicity {
				Left(periodic) => periodic.0 - ((task.0).1).1,
				Right(aperiodic) => aperiodic.0 - ((task.0).1).1,
			}
		}
		None => Duration::new(0, 0),
	}
}

// Mutators

pub fn set_state(task: &mut Task, state: State) { ((task.0).1).0 = state; }

// Samples

/// Prints the function's name and the arguments.
pub fn sample_runnable_2(args: Arguments) -> u64 {
	use crate::println;

	println!("Running sample_runnable_2");
	for element in args.iter() {
		println!("argument: {}", element);
	}

	0
}

/// Streams prime numbers on the serial port up to a limit (passed as parameter) less than 2^64
/// On my computer, find all the primes between 0 and 1.000.000 in 2:05 min
#[allow(dead_code)]
fn sample_runnable(_args: Arguments) -> u64 {
	use crate::println;
	use core::u64::MAX;
	use integer_sqrt::IntegerSquareRoot;

	println!("2");
	let mut candidate: u64 = 3;
	loop {
		if candidate == MAX {
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
			println!("{}", candidate);
		}
		candidate += 2;
	}

	0
}
