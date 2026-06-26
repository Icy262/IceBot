use std::net::TcpStream;

use crate::block::{Block, Coordinates, Direction};
use crate::movement_translator;
use crate::network::packets::{Packets, write_packet};

pub(crate) enum Movements {
	Jump(Jump),
	Walk(Walk),
	NoInput(NoInput),
}

pub(crate) fn to_packets(movement: Movements) -> Vec<Packets> {
	match movement {
		Movements::Jump(movement) => Jump::to_packets(movement),
		Movements::Walk(movement) => Walk::to_packets(movement),
		Movements::NoInput(movement) => NoInput::to_packets(movement),
	}
}

pub(crate) fn do_movement(movement: Movements, server_connection: &mut TcpStream) {
	let packets = to_packets(movement);
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
