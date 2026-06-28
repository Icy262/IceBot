//Gather a specified amount of an item. Can do this by mining or collecting from storage containers
pub(crate) struct Gather {
	pub(crate) item: String,
	pub(crate) quantity: u32,
}

impl Gather {
	pub(crate) fn get_next_behaviour(&self) -> Behaviour {
	}
}
