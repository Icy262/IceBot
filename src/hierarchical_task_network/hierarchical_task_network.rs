use crate::behaviour::actions::{Actions, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::Movements;
use crate::behaviour::movements::NoInput;
use crate::tasks::tasks::Tasks;

pub(crate) struct HierarchicalTaskNetwork {
	tasks: Vec<Tasks>,
}

impl HierarchicalTaskNetwork {
	pub(crate) fn new(task: Tasks) -> Self {
		return Self {
			tasks: vec!(task),
		}
	}

	pub(crate) fn get_next_behaviour(&mut self) -> Behaviour {
		let Some(task) = self.tasks.get_mut(0) else {
			//we must return an action that does nothing to prevent a complete HTN from crashing 
			return Behaviour {
				movement: Movements::NoInput(NoInput {}),
				action: Actions::DoNothing(DoNothing {}),
			};
		};

		//we must check if the task we just got is complete. if it's complete, remove it and call this fn recursively until we find a task that still needs doing, or the HTN is resolved
		if task.complete() {
			self.tasks.remove(0);
			return self.get_next_behaviour();
		}

		//if elementary task, return it, if not, break it down, and call this fn recursively
		return match task {
			Tasks::GoTo(task) => task.get_next_behaviour(),
			Tasks::Gather(task) => task.get_next_behaviour(),
			Tasks::FindItem(task) => task.get_next_behaviour(),
			Tasks::ClearRegion(task) => task.get_next_behaviour(),
		};
	}

	pub(crate) fn complete(&self) -> bool {
		return self.tasks.len() == 0;
	}
}
