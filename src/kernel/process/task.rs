use super::{
	ord_periodicity, Aperiodic, Constraint, Formatter, Info, Limbo, Metadata, Periodic, Result, Runnable, State,
	PRIORITY,
};
use crate::kernel::time;
use cmos_rtc::Time;
use core::{
	cmp::Ordering::{self, Equal, Greater, Less},
	fmt::Debug,
	time::Duration,
};
use either::Either::{self, Left, Right};

#[derive(Eq)]
pub struct Task {
	metadata: Metadata,
	code: Runnable,
}

impl Task {
	// constructor

	#[must_use]
	pub fn new(constraint: Constraint, code: Runnable) -> Task {
		use time::get_datetime;

		Task { metadata: (constraint, (State::Limbo(Limbo::Creating), <Duration>::new(0, 0), get_datetime())), code }
	}

	// accessors

	#[inline]
	#[must_use]
	pub fn get_metadata(&self) -> &Metadata { &self.metadata }

	#[must_use]
	#[inline]
	pub fn get_constraint(&self) -> &Constraint { &self.metadata.0 }

	#[inline]
	#[must_use]
	pub fn get_periodicity(&self) -> &Option<Either<Periodic, Aperiodic>> { &(self.metadata.0).0 }

	#[inline]
	#[must_use]
	pub fn get_runnable(&self) -> &Runnable { &self.code }

	#[inline]
	#[must_use]
	pub fn get_priority(&self) -> PRIORITY { (self.metadata.0).1 }

	#[must_use]
	#[inline]
	pub fn get_info(&self) -> &Info { &self.metadata.1 }

	#[inline]
	#[must_use]
	pub fn get_state(&self) -> State { (self.metadata.1).0 }

	#[inline]
	#[must_use]
	pub fn get_running_time(&self) -> &Duration { &(self.metadata.1).1 }

	#[inline]
	#[must_use]
	pub fn get_creation_time(&self) -> &Time { &(self.metadata.1).2 }

	#[must_use]
	pub fn get_estimated_remaining_time(&self) -> Option<Duration> {
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

	// mutators

	#[inline]
	pub fn set_state(&mut self, state: State) { (self.metadata.1).0 = state; }

	pub fn set_last_execution(&mut self, datetime: &Time) -> bool {
		if self.is_periodic() {
			self.get_periodicity().unwrap().left().unwrap().2 = *datetime;
			true
		} else {
			false
		}
	}

	#[inline]
	pub fn set_elapsed_running_time(&mut self, duration: Duration) { (self.metadata.1).1 = duration }

	// other

	#[inline]
	#[must_use]
	pub fn is_realtime(&self) -> bool { (self.metadata.0).0.is_some() }

	#[inline]
	#[must_use]
	pub fn is_periodic(&self) -> bool { self.is_realtime() && (self.metadata.0).0.as_ref().unwrap().is_left() }

	#[inline]
	#[must_use]
	pub fn is_aperiodic(&self) -> bool { self.is_realtime() && (self.metadata.0).0.as_ref().unwrap().is_right() }
}

impl Debug for Task {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(
			f,
			"Task {{ created: {:?}, state: {}, realtime: {} }}",
			self.get_creation_time(),
			self.get_state(),
			self.is_realtime()
		)
	}
}

impl PartialEq for Task {
	#[inline]
	fn eq(&self, other: &Self) -> bool { core::ptr::eq(self, other) }
}

impl Ord for Task {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self.get_periodicity(), other.get_periodicity()) {
			(Some(periodicity_a), Some(periodicity_b)) => ord_periodicity(periodicity_a, periodicity_b),
			(Some(_), None) => Greater,
			(None, Some(_)) => Less,
			(None, None) => Equal,
		}
	}
}

impl PartialOrd for Task {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
