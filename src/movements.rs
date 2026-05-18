use std::net::TcpStream;

use crate::block::{Block, Coordinates, Direction};
use crate::movement_translator;
use crate::packets::write_packet;

pub(crate) fn do_action(movement: movement_translator::Movements, server_connection: &mut TcpStream) {
	let packets = movement_translator::to_packets(movement);
	for packet in packets {
		write_packet(server_connection, packet);
	}
}

pub(crate) struct Jump {}

//makes the bot walk 0.2 blocks forward during the next tick, the conversion will handle gravity and hitboxes
pub(crate) struct Walk {
	// pub(crate) position: Coordinates, //Coordinates that the feet will occupy
}

pub(crate) struct NoInput {}
