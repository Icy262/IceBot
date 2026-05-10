pub(crate) struct Player {
	pub(crate) x: f64,
	pub(crate) y: f64,
	pub(crate) z: f64,
	pub(crate) falling: bool,
	pub(crate) facing: f64, //degrees clockwise from north
	pub(crate) elevation: f64, //degrees positive (up) or negative (down) from horizon 
	// inventory: TODO: implement
}