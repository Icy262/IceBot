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

use crate::world::World;
use crate::block::Block;
use crate::data_types::MCMetadata;
use crate::data_types::MCUByte;
use crate::block::Coordinates;

fn main() {
	thread::spawn(move || bot::bot_main("Icebot".to_string(), "localhost:25565".to_string()));
	loop {
		 println!("Blocks in world at 0, y, 0 are:");
		 for i in 0 ..= 127 {
		 	let block = World::get_block(block::Coordinates { x:0, y: i, z: 0 }).unwrap_or_else(|| Block { block_type: "Error".to_string(), metadata: MCMetadata { metadata_type: MCUByte {value:0}, value: MCUByte { value: 0 }}, position: Coordinates { x:0,y: i,z:0}});
			if block.block_type != "Error"{
				println!("{i}: {}", block.block_type);
			}
		 }

		std::thread::sleep(time::Duration::from_millis(100));
	}
	println!("Complete!");
}
