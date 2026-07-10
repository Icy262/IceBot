use std::collections::VecDeque;

use crate::behaviour::behaviour::Behaviour;
use crate::hierarchical_task_network::hierarchical_task_network::HierarchicalTaskNetwork;
use crate::scheduler::prioritised_task::{self, PrioritisedHierarchicalTaskNetwork};
use crate::tasks::tasks::Tasks;

pub(crate) struct Schedule {
	tasks: Vec<PrioritisedHierarchicalTaskNetwork>,
}

impl Schedule {
	pub(crate) fn new() -> Self {
		Self { tasks: Vec::new() }
	}

	pub(crate) fn push_task_network(
		&mut self,
		new_task: HierarchicalTaskNetwork,
		priority_function: Box<dyn FnMut() -> usize>,
	) {
		self.tasks.push(PrioritisedHierarchicalTaskNetwork {
			task: new_task,
			priority_function: priority_function,
		});
	}

	pub(crate) fn get_next_behaviour(&mut self) -> Option<Behaviour> {
		let highest_priority_task_network_index = self.get_highest_priority_task_network().ok()?;
		let highest_priority_task_network = self
			.tasks
			.get_mut(highest_priority_task_network_index)
			.expect("Should be able to find highest priority task");

		if highest_priority_task_network.task.complete() {
			self.tasks.remove(highest_priority_task_network_index);
			return self.get_next_behaviour();
		} else {
			return highest_priority_task_network.task.get_next_behaviour();
		}
	}

	fn get_highest_priority_task_network(&mut self) -> Result<usize, &'static str> {
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
