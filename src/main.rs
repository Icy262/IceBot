#![allow(warnings)]
use core::time;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;
use std::thread;

mod behaviour {
	pub(crate) mod behaviour;
	pub(crate) mod movements;
	pub(crate) mod movement_translator;
	pub(crate) mod action_translator;
	pub(crate) mod actions;
}
mod world {
	pub(crate) mod block;
	pub(crate) mod entity;
	pub(crate) mod physics;
	pub(crate) mod world;
}
mod bot;
mod item;
mod network {
	pub(crate) mod data_types;
	pub(crate) mod network_connection;
	pub(crate) mod packet_processor;
	pub(crate) mod packets;
}
mod pathfinding {
	pub(crate) mod pathfind;
	mod priority_queue;
}
mod player;
mod task;
mod tool;
mod registry {
	pub(crate) mod block_type;
}
mod scheduler {
	pub(crate) mod scheduler;
}

use crate::world::block::Block;
use crate::world::block::Coordinates;
use crate::block_type::{BlockType, build_block_type_registry};
use crate::registry::block_type;
use crate::world::world::World;

static BLOCK_REGISTRY: LazyLock<HashMap<String, BlockType>> =
	LazyLock::new(|| build_block_type_registry());

fn main() {
	thread::spawn(move || bot::bot_main("Icebot".to_string(), "localhost:25565".to_string()));
	loop {
		std::thread::sleep(time::Duration::from_millis(100));
	}
	println!("Complete!");
}
