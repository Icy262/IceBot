use crate::behaviour::behaviour::Behaviour;
use crate::world::block::Coordinates;


pub(crate) enum Tasks {
	GoTo(GoTo),
	Gather(Gather),
	FindItem(FindItem),
	ClearRegion(ClearRegion),
}

impl Tasks {
	pub(crate) fn get_next_behaviour(self) -> Behaviour {
		return match task {
			Tasks::GoTo(task) => task.get_next_behaviour(),
			Tasks::Gather(task) => task.get_next_behaviour(),
			Tasks::FindItem(task) => task.get_next_behaviour(),
			Tasks::ClearRegion(task) => task.get_next_behaviour(),
		}
	}
}
