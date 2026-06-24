#![allow(warnings)]
use core::time;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;
use std::thread;

mod action_translator;
mod actions;
mod behaviour;
mod block;
mod bot;
mod data_types;
mod entity;
mod item;
mod movement_translator;
mod movements;
mod network_connection;
mod packet_processor;
mod packets;
mod pathfinding {
	pub(crate) mod pathfind;
	mod priority_queue;
}
mod physics;
mod player;
mod task;
mod tool;
mod world;
mod registry {
	pub(crate) mod block_type;
}

use crate::block::Block;
use crate::block::Coordinates;
use crate::block_type::{BlockType, build_block_type_registry};
use crate::registry::block_type;
use crate::world::World;

static BLOCK_REGISTRY: LazyLock<HashMap<String, BlockType>> =
	LazyLock::new(|| build_block_type_registry());

fn main() {
	thread::spawn(move || bot::bot_main("Icebot".to_string(), "localhost:25565".to_string()));
	loop {
		std::thread::sleep(time::Duration::from_millis(100));
	}
	println!("Complete!");
}
