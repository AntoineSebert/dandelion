/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	06/03/2019
 */

pub mod ipc;
pub mod scheduler;
pub mod shell;
pub mod vmm;

pub mod process;

use lazy_static::lazy_static;

// https://japaric.github.io/heapless/heapless/struct.IndexSet.html

/*
use rustc_hash::FxHasher;
impl Hash for Process {
	fn hash<H: FxHasher>(&self, state: &mut H) {
		self.process_id.hash(state);
	}
}

use rustc_hash::FxHashSet;
lazy_static! {
	pub static ref PROCESS_TABLE: FxHashSet<Process> = FxHashSet::default();
}
*/
