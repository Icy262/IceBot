use crate::world::WorldUpdate;
use crate::network::{packet_processor::process_packet, packets::read_packet};
use crate::world::World;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

use byteorder::ReadBytesExt;

pub(crate) fn read_manager(stream: &mut TcpStream, tx: Sender<WorldUpdate>) {
	loop {
		let packet = read_packet(stream); //read the next packet
		if packet.is_some() {
			World::update_world_model(
				process_packet(
					packet.expect("Should not be None because we check that it is some"),
				),
				tx.clone(),
			);
		} else {
			panic!("Malformed packet");
		}
	}
}
