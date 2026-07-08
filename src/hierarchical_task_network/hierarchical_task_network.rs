use std::collections::VecDeque;

use crate::behaviour::actions::{Actions, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::Movements;
use crate::behaviour::movements::NoInput;
use crate::tasks::tasks::Tasks;

pub(crate) struct HierarchicalTaskNetwork {
	tasks: VecDeque<Tasks>,
}

impl HierarchicalTaskNetwork {
	pub(crate) fn new(task: Tasks) -> Self {
		return Self { tasks: VecDeque::from(vec![task]) };
	}

	pub(crate) fn get_next_behaviour(&mut self) -> Option<Behaviour> {
		let task = self.tasks.get_mut(0)?;

		//we must check if the task we just got is complete. if it's complete, remove it and call this fn recursively until we find a task that still needs doing, or the HTN is resolved
		if task.complete() {
			self.tasks.pop_front();
			return self.get_next_behaviour();
		}

		//get next of task. if it is a behaviour, return it, if it is a task, push it to the front of the task queue and call recursively until a behaviour is found
		task.get_next()
			.map(|next| match next {
				Next::Task(task) => {
					self.tasks.push_front(task);
					self.get_next_behaviour()
				}
				Next::Behaviour(behaviour) => Some(behaviour),
			})
			.flatten()
	}

	pub(crate) fn complete(&self) -> bool {
		return self.tasks.len() == 0;
	}
}

pub(crate) enum Next {
	Task(Tasks),
	Behaviour(Behaviour),	
}
