#![allow(dead_code)]

pub mod task;

use super::time::dt_add_du;
use alloc::{string::String, vec::Vec};
use cmos::RTCDateTime;
use core::{
	cmp::Ordering::{self, *},
	fmt::{Display, Formatter, Result},
	time::Duration,
};
use either::Either::{self, Left, Right};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
	Limbo(Limbo),
	MainMemory(MainMemory),
	SwapSpace(SwapSpace),
}

impl Display for State {
	fn fmt(&self, f: &mut Formatter) -> Result { write!(f, "{:?}", *self) }
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

pub type Arguments = Vec<String>;

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

// replace by type alias whenever possible this is just a hack to have PartialEq on Runnable
pub struct Runnable(pub fn(Arguments) -> u64);

impl PartialEq for Runnable {
	fn eq(&self, other: &Self) -> bool { self as *const _ == other as *const _ }
}
impl Eq for Runnable {}

// job

pub struct Job {
	metadata: Metadata,
	thread: Vec<Runnable>,
}

// group

pub struct Group {
	member: Vec<task::Task>,
}

// order

pub fn ord_periodicity(a: &Either<Periodic, Aperiodic>, b: &Either<Periodic, Aperiodic>) -> Ordering {
	match (a, b) {
		(Right(aperiodic_a), Right(aperiodic_b)) => aperiodic_a.1.cmp(&aperiodic_b.1),
		(Right(aperiodic_a), Left(periodic_b)) => ord_p_ap(&periodic_b, &aperiodic_a),
		(Left(periodic_a), Right(aperiodic_b)) => ord_p_ap(&periodic_a, &aperiodic_b),
		(Left(periodic_a), Left(periodic_b)) => ord_p_p(&periodic_a, &periodic_b),
	}
}

// to check
pub fn ord_p_ap(a: &Periodic, b: &Aperiodic) -> Ordering {
	match dt_add_du(a.2, a.1) {
		Some(deadline_a) => {
			if deadline_a < b.1 {
				Less
			} else if b.1 < deadline_a {
				Greater
			} else {
				Equal
			}
		}
		None => Less,
	}
}

// to check
pub fn ord_p_p(a: &Periodic, b: &Periodic) -> Ordering {
	match (dt_add_du(a.2, a.1), dt_add_du(b.2, b.1)) {
		(Some(deadline_a), Some(deadline_b)) => deadline_a.cmp(&deadline_b),
		(Some(_), None) => Greater,
		(None, Some(_)) => Less,
		(None, None) => Equal,
	}
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
