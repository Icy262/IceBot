#![allow(warnings)]
use core::time;
use std::collections::HashMap;
use std::str::FromStr;
use std::thread;
use std::sync::LazyLock;

mod action_translator;
mod actions;
mod block;
mod data_types;
mod entity;
mod network_connection;
mod packet_processor;
mod packets;
mod world;
mod player;
mod bot;
mod movements;
mod movement_translator;
mod physics;
mod task;
mod behaviour;
mod item;
mod tool;
mod registry {
	pub(crate) mod block_type;
}

use crate::registry::block_type;
use crate::world::World;
use crate::block::Block;
use crate::block::Coordinates;
use crate::block_type::{BlockType, build_block_type_registry};

static BLOCK_REGISTRY: LazyLock<HashMap<String, BlockType>> = LazyLock::new(|| build_block_type_registry());

fn main() {
	thread::spawn(move || bot::bot_main("Icebot".to_string(), "localhost:25565".to_string()));
	loop {
		std::thread::sleep(time::Duration::from_millis(100));
	}
	println!("Complete!");
}
