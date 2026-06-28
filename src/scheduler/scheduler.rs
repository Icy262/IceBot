use std::collections::VecDeque;

use crate::behaviour::behaviour::Behaviour;
use crate::scheduler::prioritised_task::{self, PrioritisedTask};
use crate::tasks::tasks::Tasks;

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

	pub(crate) fn push_task(
		&mut self,
		new_task: Tasks,
		priority_function: Box<dyn FnMut() -> usize>,
	) {
		self.tasks.push(PrioritisedTask {
			task: new_task,
			priority_function: priority_function,
		});
	}

	pub(crate) fn get_next_behaviour(&mut self) -> Option<Behaviour> {
		let highest_priority_task_index = self.get_highest_priority_task().unwrap_or(return None);
		let highest_priority_task = self
			.tasks
			.get(highest_priority_task_index)
			.expect("Should be able to find highest priority task");

		if highest_priority_task.task.complete() {
			self.tasks.remove(highest_priority_task_index);
			self.current_task = None;
			return self.get_next_behaviour();
		} else {
			return Some(highest_priority_task.task.get_next_behaviour());
		}
	}

	fn get_highest_priority_task(&mut self) -> Result<usize, &'static str> {
		let mut highest_priority_value = None;
		let mut highest_priority_index = None;

		for (index, task) in self.tasks.iter_mut().enumerate() {
			match highest_priority_index {
				Some(highest_priority) => {
					let task_priority = (task.priority_function)();

					if task_priority > highest_priority {
						highest_priority_value = Some(task_priority);
						highest_priority_index = Some(index);
					}
				}
				None => {
					highest_priority_value = Some((task.priority_function)());
					highest_priority_index = Some(index);
				}
			}
		}

		return match highest_priority_index {
			Some(highest_priority_index) => Ok(highest_priority_index),
			None => Err("Task queue is empty"),
		};
	}
}
