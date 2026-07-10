use crate::behaviour;
use crate::behaviour::behaviour::Behaviour;
use crate::hierarchical_task_network::hierarchical_task_network::Next;
use crate::tasks::{clear_region::ClearRegion, find_item::FindItem, gather::Gather, go_to::GoTo};

pub(crate) enum Tasks {
	GoTo(GoTo),
	Gather(Gather),
	FindItem(FindItem),
	ClearRegion(ClearRegion),
}

impl Tasks {
	pub(crate) fn get_next(&mut self) -> Option<Next> {
		match self {
			Tasks::GoTo(task) => task.get_next(),
			Tasks::Gather(task) => task.get_next(),
			Tasks::FindItem(task) => task.get_next(),
			Tasks::ClearRegion(task) => task.get_next(),
		}
	}

	pub(crate) fn complete(&self) -> bool {
		return match self {
			Tasks::GoTo(task) => task.complete(),
			Tasks::Gather(task) => task.complete(),
			Tasks::FindItem(task) => task.complete(),
			Tasks::ClearRegion(task) => task.complete(),
		};
	}
}
