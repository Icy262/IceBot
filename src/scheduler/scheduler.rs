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

	fn get_highest_priority_task(self) -> Result<u32, &'static str> {
		return Ok(
			self
				.tasks
				.iter()
				.enumerate()
				.min_by(|(_, &prioritised_task)| (prioritised_task.priority_function)())
				.ok_or("no tasks available")?
				.0
			as u32
		);
	}
}
