use std::collections::VecDeque;

use crate::task::Tasks;

pub(crate) struct Schedule {
	tasks: Vec<Tasks>,
}

impl Schedule {
	pub(crate) fn new() -> Self {
		Self {
			tasks: Vec::new(),
		}
	}

	pub(crate) fn push_task(&mut self, new_task: Tasks) {
		self.tasks.push(value);
	}
}