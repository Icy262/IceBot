use std::collections::HashMap;

use yaml_rust::{Yaml, YamlLoader};

use crate::item::ItemType;
use crate::registry::block_type::LightTransparency::Transparent;
use crate::tool::ToolType;
use crate::world::block;

pub(crate) struct BlockType {
	pub(crate) friendly_name: String,
	pub(crate) id: String,
	//TODO: implement later when required
	//pub(crate) tool: ToolType,
	//pub(crate) min_tier: ,
	//pub(crate) hardness: u32,
	//pub(crate) drops: HashMap<ToolType, ItemType>,
	//pub(crate) hitbox: Hitbox,
	pub(crate) collision: Collision,
	pub(crate) transparency: LightTransparency,
}

//TODO: implement
pub(crate) struct Hitbox {}

impl BlockType {
	//converts the type to a user friendly name
	fn id_to_friendly_name(block_id: String) {
		//TODO: implement
	}

	//converts the user friendly name to its associated in game type
	fn friendly_name_to_id(friendly_name: String) {
		//TODO: implement
	}
}
pub(crate) enum Collision {
	Solid,
	NonSolid,
	Liquid,
}

pub(crate) enum LightTransparency {
	Opaque,
	Transparent,
}

//Builds a registry of all the block types.
pub(crate) fn build_block_type_registry() -> HashMap<String, BlockType> {
	static BLOCK_TYPE_DEFINITIONS: &str = include_str!(concat!(
		"../../data/",
		env!("GAME_VERSION"),
		"/game_data/block_types.yaml"
	));

	let yaml_block_type_definitions = YamlLoader::load_from_str(BLOCK_TYPE_DEFINITIONS)
		.expect("Should be able to convert string of yaml to yaml object")[0]["Block Types"]
		.to_owned();

	let mut block_type_registry = HashMap::new();

	for block_type in yaml_block_type_definitions {
		//TODO: implement better error messages (state which block is broken)
		block_type_registry.insert(
			block_type["id"]
				.as_i64()
				.expect("Should be able to convert yaml representation of block type id to i64")
				.to_string(),
			BlockType {
				friendly_name: block_type["friendly name"]
					.as_str()
					.expect(
						"Should be able to convert yaml representation of block friendly name to str",
					)
					.to_string(),
				id: block_type["id"]
					.as_i64()
					.expect("Should be able to convert yaml representation of block type id to i64")
					.to_string(),
				collision: match block_type["collision"].as_str().expect(
					"Should be able to convert yaml representation of block type collision to str",
				) {
					"solid" => Collision::Solid,
					"non-solid" => Collision::NonSolid,
					"liquid" => Collision::Liquid,
					_ => panic!("Invalid collision type in block types yaml"),
				},
				transparency: match block_type["transparency"].as_str().expect(
					"Should be able to convert yaml representation of block type transparency to str",
				) {
					"opaque" => LightTransparency::Opaque,
					"transparent" => LightTransparency::Transparent,
					_ => panic!("Invalid transparency type in block types yaml"),
				},
			},
		);
	}

	return block_type_registry;
}
