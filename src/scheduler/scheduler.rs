use std::collections::VecDeque;

use crate::task::Tasks;

pub(crate) struct Schedule {
	tasks: VecDeque<Tasks>,
}

impl Schedule {
	pub(crate) fn new() -> Self {
		Self {
			tasks: VecDeque::new(),
		}
	}
}