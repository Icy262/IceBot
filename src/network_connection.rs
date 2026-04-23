use std::net::TcpStream;
use byteorder::ReadBytesExt;
use crate::packets::read_packet;

fn read_manager(stream: &mut TcpStream) {
	loop {
		if let Ok(id) = stream.read_u8() {
			let packet = read_packet(stream, id);
			//TODO: do something with packet
		} else {
			return;
		}
	}
}