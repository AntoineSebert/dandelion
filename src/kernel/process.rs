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
pub type Periodic = (Duration, Duration, RTCDateTime); // make tuple struct

/// 0 : estimated completion time
/// 1 : deadline
/// 2 : start delay
pub type Aperiodic = (Duration, RTCDateTime, Option<RTCDateTime>); // make tuple struct

/// 0 : process state
/// 1 : elapsed running time
/// 2 : creation time
pub type Info = (State, Duration, RTCDateTime); // make tuple struct
pub type Constraint = (Option<Either<Periodic, Aperiodic>>, PRIORITY); // make tuple struct

pub type Metadata = (Constraint, Info); // make tuple struct

pub type Runnable = fn(Arguments) -> u64;

//#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Task {
	metadata: Metadata,
	code: Runnable,
}

pub type Job<'a> = (Metadata, &'a [&'a Runnable]);
pub type Group<'a> = &'a [&'a Task];

// Accessors

impl Task {
	pub fn new(constraint: Constraint, code: Runnable) -> Task {
		use super::time::get_datetime;

		Task { metadata: (constraint, (State::Limbo(Limbo::Creating), <Duration>::new(0, 0), get_datetime())), code }
	}

	#[inline]
	pub fn get_metadata(&self) -> &Metadata { &self.metadata }

	#[inline]
	pub fn get_constraint(&self) -> &Constraint { &self.metadata.0 }

	#[inline]
	pub fn get_periodicity(&self) -> &Option<Either<Periodic, Aperiodic>> { &(self.metadata.0).0 }

	#[inline]
	pub fn get_runnable(&self) -> &Runnable { &self.code }

	#[inline]
	pub fn get_priority(&self) -> PRIORITY { (self.metadata.0).1 }

	#[inline]
	pub fn get_info(&self) -> &Info { &self.metadata.1 }

	#[inline]
	pub fn get_state(&self) -> State { (self.metadata.1).0 }

	#[inline]
	pub fn get_running_time(&self) -> &Duration { &(self.metadata.1).1 }

	#[inline]
	pub fn get_creation_time(&self) -> &RTCDateTime { &(self.metadata.1).2 }

	pub fn get_estimated_remaining_time(&self) -> Option<Duration> {
		use Either::{Left, Right};

		match self.get_periodicity() {
			Some(periodicity) => {
				match periodicity {
					Left(periodic) => Some(periodic.0 - (self.metadata.1).1),
					Right(aperiodic) => Some(aperiodic.0 - (self.metadata.1).1),
				}
			}
			None => None,
		}
	}

	// Mutators

	#[inline]
	pub fn set_state(&mut self, state: State) { (self.metadata.1).0 = state; }
}

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

/// Streams prime numbers on the serial port up to a limit (passed as parameter) less than 2^64.
/// On my computer, finds all the primes between 0 and 1.000.000 in 2:05 min.
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
