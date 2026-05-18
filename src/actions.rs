use std::net::TcpStream;

use crate::block::{Block, Coordinates, Direction};
use crate::action_translator;
use crate::packets::write_packet;

pub(crate) fn do_action(action: action_translator::Actions, server_connection: &mut TcpStream) {
	let packets = action_translator::to_packets(action);
	for packet in packets {
		write_packet(server_connection, packet);
	}
}

//TODO: implement support for online servers
pub(crate) struct Join {
	pub(crate) username: String, //player username
}

pub(crate) struct Look {
	pub(crate) target: Coordinates, //Coordinates that bot will look at
}

pub(crate) struct BreakBlock {
	pub(crate) position: Coordinates, //coordinates of the block to break
	pub(crate) face: Direction, //face of block we are hitting
}

//For if we just want a block in a particular position
pub(crate) struct PlaceBlock {
	pub(crate) position: Coordinates, //position of the block's location
	pub(crate) rotation: Direction, //direction of the placed block
}

//For if we want to specify the block we are placing against
pub(crate) struct PlaceBlockAgainst {
	pub(crate) position: Coordinates, //position of the block we are placing against
	pub(crate) rotation: Direction, //direction of the placed block
}
