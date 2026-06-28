use crate::behaviour::behaviour::Behaviour;
use crate::world::block::Coordinates;


pub(crate) enum Tasks {
	GoTo(GoTo),
	Gather(Gather),
	FindItem(FindItem),
	ClearRegion(ClearRegion),
}

impl Tasks {
	pub(super) fn activate(self) {
		return match task {
			Tasks::GoTo(task) => task.activate(),
			Tasks::Gather(task) => task.activate(),
			Tasks::FindItem(task) => task.activate(),
			Tasks::ClearRegion(task) => task.activate(),
		}
	}

	pub(super) fn deactivate(self) {
		return match task {
			Tasks::GoTo(task) => task.deactivate(),
			Tasks::Gather(task) => task.deactivate(),
			Tasks::FindItem(task) => task.deactivate(),
			Tasks::ClearRegion(task) => task.deactivate(),
		}
	}

	pub(crate) fn get_next_behaviour(self) -> Behaviour {
		return match task {
			Tasks::GoTo(task) => task.get_next_behaviour(),
			Tasks::Gather(task) => task.get_next_behaviour(),
			Tasks::FindItem(task) => task.get_next_behaviour(),
			Tasks::ClearRegion(task) => task.get_next_behaviour(),
		}
	}
}
