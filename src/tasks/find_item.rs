use crate::behaviour::actions::{Actions, DoNothing};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::{Movements, NoInput};
use crate::hierarchical_task_network::hierarchical_task_network::Next;

//Find a single item by mining or collecting from storage containers. Could be called repeatedly by Gather until the quantity is fufiled
pub(crate) struct FindItem {
	pub(crate) item: String,
}

impl FindItem {
	pub(crate) fn get_next(&mut self) -> Option<Next> {
		return None;
	}

	pub(crate) fn complete(&self) -> bool {
		return false;
	}
}
