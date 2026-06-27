use std::collections::VecDeque;

use crate::{scheduler::prioritised_task::PrioritisedTask, task::Tasks};

pub(crate) struct Schedule {
	tasks: Vec<PrioritisedTask>,
}

impl Schedule {
	pub(crate) fn new() -> Self {
		Self {
			tasks: Vec::new(),
		}
	}

	pub(crate) fn push_task(&mut self, new_task: Tasks, priority_function: Box<dyn FnMut() -> u32>) {
		self.tasks.push(
			PrioritisedTask {
				task: new_task,
				priority_function: priority_function,
			}
		);
	}
}