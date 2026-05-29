#![allow(warnings)]
use core::time;
use std::str::FromStr;
use std::thread;

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

use crate::world::World;
use crate::block::Block;
use crate::data_types::MCMetadata;
use crate::data_types::MCUByte;
use crate::block::Coordinates;

fn main() {
	thread::spawn(move || bot::bot_main("Icebot".to_string(), "localhost:25565".to_string()));
	loop {
		std::thread::sleep(time::Duration::from_millis(100));
	}
	println!("Complete!");
}
