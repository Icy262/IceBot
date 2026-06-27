use crate::behaviour::{actions::Actions, movements::Movements};

//Represents the movement and action of a bot in a single tick
pub(crate) struct Behaviour {
	pub(crate) movement: Movements,
	pub(crate) action: Actions,
}
