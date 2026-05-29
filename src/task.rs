use crate::{behaviour::Behaviour, block::Coordinates};

pub(crate) enum Tasks {
	GoTo(GoTo),
	Gather(Gather),
	FindItem(FindItem),
	ClearRegion(ClearRegion),
}

pub(crate) fn to_Movement(task: Tasks) -> Vec<Behaviour> {
	match task {
		Tasks::GoTo(task) => GoTo::to_behaviour(task),
		Tasks::Gather(task) => Gather::to_behaviour(task),
		Tasks::FindItem(task) => FindItem::to_behaviour(task),
		Tasks::ClearRegion(task) => ClearRegion::to_behaviour(task),
	}
}

//Pathfind to a position and generate the required actions to get there
pub(crate) struct GoTo {
	pub(crate) position: Coordinates,
}

//Gather a specified amount of an item. Can do this by mining or collecting from storage containers
pub(crate) struct Gather {
	pub(crate) item: String,
	pub(crate) quantity: u32,
}

//Find a single item by mining or collecting from storage containers. Could be called repeatedly by Gather until the quantity is fufiled
pub(crate) struct FindItem {
	pub(crate) item: String,
}

//Remove all the blocks from a given region
pub(crate) struct ClearRegion {
	pub(crate) start_corner: Coordinates,
	pub(crate) end_corner: Coordinates,
}
