use crate::bot::PLAYER;
use crate::network::data_types::{MCMetadata, MCUByte};
use crate::world::block::{Block, Coordinates};
use crate::world::entity::{Entity, EntityPositionAndLook};
use std::collections::HashMap;
use std::mem::drop;
use std::sync::LazyLock;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, MutexGuard, RwLock};

pub(crate) static WORLD_MODEL: LazyLock<World> = LazyLock::new(|| World {
	chunks: Mutex::new(HashMap::new()),
	entities: Vec::new(),
});

pub(crate) struct Region {
	//region is an area inside of a chunk (does not cross chunk borders)
	pub(crate) start_x: i32, //start coordinates
	pub(crate) start_y: i32,
	pub(crate) start_z: i32,
	pub(crate) size_x: i32, //distance the region extends
	pub(crate) size_y: i32,
	pub(crate) size_z: i32,
	pub(crate) palette: Vec<String>, //array of standard ids. The id at index n maps to the local id n
	//TODO: consider using 1d vec for performance
	pub(crate) local_ids: Vec<Vec<Vec<u8>>>, //3d array of local ids (x,y,z)
	                                         // metadata: Vec<MCMetadata>, //TODO: implement
}

pub(crate) struct Chunk {
	//use u8s to store the local block id
	//TODO: implement support for using larger data types for local ids if there are not enough available. consider using byte packing to reduce memory footprint
	pub(crate) x: i32, //chunk coordinate
	pub(crate) z: i32,
	palette: Vec<String>, //array of standard ids. The id at index n maps to the local id n
	//TODO: consider using 1d vec for performance
	local_ids: Vec<Vec<Vec<u8>>>, //array of local ids
	                              // metadata: Vec<MCMetadata>, //TODO: implement
}

pub(crate) struct World {
	//TODO: implement trimming the loaded blocks
	chunks: Mutex<HashMap<(i32, i32), Arc<Mutex<Chunk>>>>, //for performance, we don't use a regular Block as these have too much overhead. instead, break the world into identically sized chunks which can store blocks more efficiently using palettes. use a hashmap so we can store an arbitrary number of chunks that may be far apart in the world. the hashmap key is formed as "x chunk coordinate,y chunk coordinate"
	entities: Vec<Entity>,                                 //entities in the world
}

impl World {
	pub(crate) fn update_world_model(update: WorldUpdate, tx: Sender<WorldUpdate>) {
		match update {
			WorldUpdate::SingleBlock(block) => {
				//get a lock on the chunk
				let chunk = Self::get_chunk_of_block(&block.position);

				//if the chunk is not loaded, it does not make sense to update a single block, so return None
				if chunk.is_none() {
					return;
				}

				//expect the chunk because it is Some, then get a lock on it
				let chunk = chunk.expect("Should not be None, because we just checked");
				let mut chunk = chunk.lock().expect("Critical error in getting a chunk");

				//get the local x, y, z coordinates (position relative to the lowest global coordinate corner of the chunk)
				let local_coordinates = Self::global_to_local_coordinates(&Coordinates {
					x: block.position.x,
					y: block.position.y,
					z: block.position.z,
				});

				//overwrite the current block with the new one
				dbg!(local_coordinates.y);
				chunk.local_ids[local_coordinates.x as usize][local_coordinates.y as usize]
					[local_coordinates.z as usize] =
					Self::get_or_push_local_id(&block.block_id.to_string(), &mut chunk);
			}
			WorldUpdate::MultiBlock(blocks) => {
				blocks.into_iter().for_each(|block| {
					Self::update_world_model(WorldUpdate::SingleBlock(block), tx.clone())
				});
			}
			WorldUpdate::BlockRegion(region) => {
				let chunk_x = region.start_x.div_euclid(16);
				let chunk_z = region.start_z.div_euclid(16);

				//get a lock on the world
				let mut world = WORLD_MODEL
					.chunks
					.lock()
					.expect("Critical error in getting lock on world model");

				//get the chunk the region is updating or create a new one
				let chunk = world
					.entry((chunk_x, chunk_z))
					//if the chunk is not currently loaded, create a new chunk and push it to the hashmap
					.or_insert_with(|| {
						Arc::new(Mutex::new(Chunk {
							x: chunk_x,
							z: chunk_z,
							palette: Vec::new(),
							local_ids: vec![vec![vec![0u8; 16]; 128]; 16],
						}))
					})
					.clone();

				//release lock on the world
				drop(world);

				//get a lock on the chunk
				let mut chunk = chunk
					.lock()
					.expect("Critical error in updating world model");

				//get the local (chunk) coordinates of the orgin of the region
				let region_orgin_local_coordinates =
					Self::global_to_local_coordinates(&Coordinates {
						x: region.start_x,
						y: region.start_y,
						z: region.start_z,
					});

				//extract the local coordinates from the struct
				let region_local_orgin_x = region_orgin_local_coordinates.x as usize;
				let region_local_orgin_y = region_orgin_local_coordinates.y as usize;
				let region_local_orgin_z = region_orgin_local_coordinates.z as usize;

				//iterate over each block in the region
				for current_region_offset_x in 0..region.size_x as usize {
					for current_region_offset_y in 0..region.size_y as usize {
						for current_region_offset_z in 0..region.size_z as usize {
							//get the old local id, convert to the global id, and then convert the global id to a new local id
							let old_local_id = region.local_ids[current_region_offset_x]
								[current_region_offset_y][current_region_offset_z];
							let global_id = region.palette.get(old_local_id as usize).expect(
								"Should not be None because that would make the region invalid",
							);
							let new_local_id = Self::get_or_push_local_id(global_id, &mut chunk);

							chunk.local_ids[region_local_orgin_x + current_region_offset_x]
								[region_local_orgin_y + current_region_offset_y]
								[region_local_orgin_z + current_region_offset_z] = new_local_id;
						}
					}
				}
			}
			//TODO: implement for entities
			WorldUpdate::SingleEntity(entity) => (),
			WorldUpdate::MultiEntity(entity) => (),
			WorldUpdate::PlayerUpdate(position_and_look) => {
				let _ = tx.send(WorldUpdate::PlayerUpdate(position_and_look));
			}
			WorldUpdate::NoEffect => (),
		}
	}

	pub(crate) fn get_chunk(chunk_x: i32, chunk_z: i32) -> Option<Arc<Mutex<Chunk>>> {
		//get a lock on the world
		let world = WORLD_MODEL
			.chunks
			.lock()
			.expect("Critical failure in obtaining world model");

		//get the chunk and return an Arc to it
		return world.get(&(chunk_x, chunk_z)).cloned();
	}

	pub(crate) fn get_chunk_of_block(position: &Coordinates) -> Option<Arc<Mutex<Chunk>>> {
		//find the chunk coordinates
		let chunk_x = position.x.div_euclid(16);
		let chunk_z = position.z.div_euclid(16);

		//return the chunk
		return Self::get_chunk(chunk_x, chunk_z);
	}

	fn get_or_push_local_id(block_id: &String, chunk: &mut MutexGuard<Chunk>) -> u8 {
		//get the local id
		let mut local_block_id = chunk.palette.iter().position(|block| block == block_id);

		//if Some, then it was in the palette and we have the local id, if None, we should push the block type to the palette and use the index of the final element as the local id
		if local_block_id.is_none() {
			chunk.palette.push(block_id.clone());
			return (chunk.palette.len() - 1) as u8;
		} else {
			return local_block_id.expect("Should not be None because we checked") as u8;
		}
	}

	pub(crate) fn get_block(position: Coordinates) -> Option<Block> {
		//get the chunk
		let chunk = Self::get_chunk_of_block(&position);

		//if the chunk is not loaded, we cannot get the block, so return None
		if chunk.is_none() {
			return None;
		}

		//unwrap the chunk
		let chunk = chunk.expect("Should not be None because we just checked that");

		//get a lock on the chunk
		let chunk = chunk.lock().expect("Critical error in reading world state");

		//get the local (chunk) coordinates of the block
		let region_orgin_local_coordinates = Self::global_to_local_coordinates(&position);

		//extract the local coordinates from the struct
		let local_x = region_orgin_local_coordinates.x as usize;
		let local_y = region_orgin_local_coordinates.y as usize;
		let local_z = region_orgin_local_coordinates.z as usize;

		return Some(Block {
			block_id: chunk
				.palette
				.get(chunk.local_ids[local_x][local_y][local_z] as usize)
				.expect("Should not fail because that would make the chunk corrupted")
				.to_owned(),
			position: position,
		});
	}

	fn global_to_local_coordinates(global_coordinates: &Coordinates) -> Coordinates {
		//get convert to local coordinates
		Coordinates {
			x: global_coordinates.x & 15, //and with 15 to get the (positive) remainder for the global coordinate, which is the coordinate relative to the orgin of the chunk
			y: global_coordinates.y,
			z: global_coordinates.z & 15,
		}
	}

	//TODO: implement
	pub(crate) fn get_entity(entity: Entity) {}

	pub(crate) fn get_all_entity(entity: Entity) {}
}

pub(crate) enum WorldUpdate {
	SingleBlock(Block),
	MultiBlock(Vec<Block>),
	BlockRegion(Region),
	SingleEntity(Entity),
	MultiEntity(Vec<Entity>),
	PlayerUpdate(EntityPositionAndLook),
	NoEffect,
}
