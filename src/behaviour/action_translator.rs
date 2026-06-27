use crate::behaviour::actions::Actions;
use crate::network::packets::*;
use crate::network::data_types::*;

use crate::behaviour::actions::Join;
impl Join {
	pub(crate) fn to_packets(action: Join) -> Vec<Packets> {
let mut packets: Vec<Packets> = Vec::new();
let username = MCString16 {
    length: MCShort { value: action.username.len() as i16 },
    text: action.username,
};
packets.push(
    Packets::Handshake(
        Handshake {
            username: username.clone(),
        }
    )
);
packets.push(
    Packets::LoginRequest(
        LoginRequest {
            protocol_version: MCInt { value: 14 },
            username: username,
            map_seed: MCLong { value: 0 as i64 },
            dimension: MCByte { value: 0 as i8 },
        }
    )
);
return packets;
	}
}

use crate::behaviour::actions::DoNothing;
impl DoNothing {
	pub(crate) fn to_packets(action: DoNothing) -> Vec<Packets> {
let mut packets: Vec<Packets> = Vec::new();
return packets;
	}
}

use crate::behaviour::actions::Look;
impl Look {
	pub(crate) fn to_packets(action: Look) -> Vec<Packets> {
//TODO: implement
let mut packets: Vec<Packets> = Vec::new();
return packets;
	}
}

use crate::behaviour::actions::PlaceBlock;
impl PlaceBlock {
	pub(crate) fn to_packets(action: PlaceBlock) -> Vec<Packets> {
//TODO: implement
let mut packets: Vec<Packets> = Vec::new();
return packets;
	}
}

use crate::behaviour::actions::PlaceBlockAgainst;
impl PlaceBlockAgainst {
	pub(crate) fn to_packets(action: PlaceBlockAgainst) -> Vec<Packets> {
//TODO: implement
let mut packets: Vec<Packets> = Vec::new();
return packets;	}
}

