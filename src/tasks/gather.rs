use crate::behaviour::actions::{Actions, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::{Movements, NoInput};
use crate::hierarchical_task_network::hierarchical_task_network::Next;

//Gather a specified amount of an item. Can do this by mining or collecting from storage containers
pub(crate) struct Gather {
	pub(crate) item: String,
	pub(crate) quantity: u32,
}

impl Gather {
	pub(crate) fn get_next(&mut self) -> Option<Next> {
		return None;
	}

	pub(crate) fn complete(&self) -> bool {
		return false;
	}
}
