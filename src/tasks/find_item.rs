use crate::behaviour::behaviour::Behaviour;

//Find a single item by mining or collecting from storage containers. Could be called repeatedly by Gather until the quantity is fufiled
pub(crate) struct FindItem {
	pub(crate) item: String,
}

impl FindItem {
	pub(crate) fn get_next_behaviour(&self) -> Behaviour {
	}
	
	pub(crate) fn complete(self) -> bool {
	}
}
