use crate::action_translator;
use crate::block::{Block, Coordinates, Direction};
use crate::network::packets::{Packets, write_packet};
use std::net::TcpStream;

pub(crate) enum Actions {
	Join(Join),
	DoNothing(DoNothing),
	Look(Look),
	PlaceBlock(PlaceBlock),
	PlaceBlockAgainst(PlaceBlockAgainst),
}

pub(crate) fn to_packets(action: Actions) -> Vec<Packets> {
	match action {
		Actions::Join(action) => Join::to_packets(action),
		Actions::DoNothing(action) => DoNothing::to_packets(action),
		Actions::Look(action) => Look::to_packets(action),
		Actions::PlaceBlock(action) => PlaceBlock::to_packets(action),
		Actions::PlaceBlockAgainst(action) => PlaceBlockAgainst::to_packets(action),
	}
}

pub(crate) fn do_action(action: Actions, server_connection: &mut TcpStream) {
	let packets = to_packets(action);
	for packet in packets {
		write_packet(server_connection, packet);
	}
}

//TODO: implement support for online servers
pub(crate) struct Join {
	pub(crate) username: String, //player username
}

pub(crate) struct DoNothing {}

pub(crate) struct Look {
	pub(crate) target: Coordinates, //Coordinates that bot will look at
}

pub(crate) struct BreakBlock {
	pub(crate) position: Coordinates, //coordinates of the block to break
	pub(crate) face: Direction,       //face of block we are hitting
}

//For if we just want a block in a particular position
pub(crate) struct PlaceBlock {
	pub(crate) position: Coordinates, //position of the block's location
	pub(crate) rotation: Direction,   //direction of the placed block
}

//For if we want to specify the block we are placing against
pub(crate) struct PlaceBlockAgainst {
	pub(crate) position: Coordinates, //position of the block we are placing against
	pub(crate) rotation: Direction,   //direction of the placed block
}
