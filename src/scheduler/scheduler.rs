use std::collections::VecDeque;

use crate::{scheduler::prioritised_task::PrioritisedTask, task::Tasks};

pub(crate) struct Schedule {
	//index of the highest priority task. None means there is no active task, which would happen if we run out of tasks.
	current_task: Option<usize>,
	tasks: Vec<PrioritisedTask>,
}

impl Schedule {
	pub(crate) fn new() -> Self {
		Self {
			current_task: None,
			tasks: Vec::new(),
		}
	}

	pub(crate) fn push_task(&mut self, new_task: Tasks, priority_function: Box<dyn FnMut() -> usize>) {
		self.tasks.push(
			PrioritisedTask {
				task: new_task,
				priority_function: priority_function,
			}
		);
	}

	pub(crate) fn get_next_behaviour(&mut self) {
		let highest_priority_task_index = self.get_highest_priority_task().unwrap_or(return);
		let highest_priority_task = self
			.tasks
			.get(highest_priority_task_index)
			.expect("Should be able to find highest priority task to activate it");

		if let Some(current_task_index) = self.current_task {
			if current_task_index != highest_priority_task_index {
				self
					.tasks
					.get(current_task_index)
					.expect("Should be able to find old task to deactivate it")
					.deactivate();

				highest_priority_task.activate();
			}
		} else {  

			highest_priority_task.activate();
		}

		if highest_priority_task.complete() {
			self.tasks.remove(highest_priority_task_index);
			self.current_task = None;
			return self.get_next_behaviour();
		} else {
			return highest_priority_task.get_next_behaviour();
		}
	}

	fn get_highest_priority_task(self) -> Result<usize, &'static str> {
		return Ok(
			self
				.tasks
				.iter()
				.enumerate()
				.min_by(|(_, &prioritised_task)| (prioritised_task.priority_function)())
				.ok_or("no tasks available")?
				.0
		);
	}
}
