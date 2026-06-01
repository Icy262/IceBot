#[derive(Copy, Clone)]
pub(crate) struct Player {
	pub(crate) x: f64,
	pub(crate) y: f64,
	pub(crate) z: f64,
	//Velocity is in blocks per tick
	pub(crate) vx: f64,
	pub(crate) vy: f64,
	pub(crate) vz: f64,
	pub(crate) on_ground: bool,
	pub(crate) yaw: f64, //degrees clockwise from north
	pub(crate) pitch: f64, //degrees positive (up) or negative (down) from horizon 
	//TODO: implement inventory
}