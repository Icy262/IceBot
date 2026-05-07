use crate::data_types::MCMetadata;

pub(crate) struct Entity {
	pub(crate) block_type: String,
	pub(crate) metadata: MCMetadata,
	pub(crate) x: f64,
	pub(crate) y: f64,
	pub(crate) z: f64,
}
