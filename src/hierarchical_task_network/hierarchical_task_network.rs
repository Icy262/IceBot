use crate::behaviour::actions::{Actions, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::Movements;
use crate::behaviour::movements::NoInput;
use crate::tasks::tasks::Tasks;

pub(crate) struct HierarchicalTaskNetwork {
	tasks: Vec<Tasks>,
}

impl HierarchicalTaskNetwork {
	pub(crate) fn get_next_behaviour(&mut self) -> Behaviour {
		let Some(task) = self.tasks.get_mut(0) else {
			return Behaviour {
				movement: Movements::NoInput(NoInput {}),
				action: Actions::DoNothing(DoNothing {}),
			};
		};

		//if elementary task, return it, if not, break it down, and call this fn recursively
		return match task {
			Tasks::GoTo(task) => task.get_next_behaviour(),
			Tasks::Gather(task) => task.get_next_behaviour(),
			Tasks::FindItem(task) => task.get_next_behaviour(),
			Tasks::ClearRegion(task) => task.get_next_behaviour(),
		};
	}
}
