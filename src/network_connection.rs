use crate::{packet_processor::process_packet, packets::read_packet, world::World};
use std::net::TcpStream;

use byteorder::ReadBytesExt;

pub(crate) fn read_manager(stream: &mut TcpStream) {
	loop {
		let packet = read_packet(stream); //read the next packet
		if packet.is_some() {
			World::update_world_model(process_packet(packet.expect("Should not be None because we check that it is some")));
		} else {
			panic!("Malformed packet");
		}
	}
}
