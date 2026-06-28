use core::time;
use std::collections::VecDeque;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::{cell::RefCell, collections::LinkedList};

use crate::behaviour;
use crate::behaviour::actions::{Actions, DoNothing, Join, Look, do_action, to_packets};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::{Jump, Movements, NoInput, Walk, do_movement};
use crate::network::data_types::{MCBool, MCDouble, MCFloat};
use crate::network::data_types::{MCMetadata, MCUByte};
use crate::network::network_connection;
use crate::network::packets::write_packet;
use crate::network::packets::{KeepAlive, Packets, PlayerPositionandLook};
use crate::player::Player;
use crate::world::block::{self, Block, Coordinates};
use crate::world::world::{World, WorldUpdate};

thread_local! {
	pub(crate) static PLAYER: RefCell<Player> = RefCell::new(
		Player {
			x: 0f64,
			y: 0f64,
			z: 0f64,
			vx: 0f64,
			vy: 0f64,
			vz: 0f64,
			on_ground: false,
			yaw: 0f64,
			pitch: 0f64,
		}
	);
}

pub(crate) fn bot_main(username: String, server: String) {
	let mut server_connection = TcpStream::connect(server).unwrap();

	//Join the server
	let join = Join { username: username };
	let packets = Join::to_packets(join);
	for packet in packets {
		write_packet(&mut server_connection, packet);
	}

	//Start a network manager
	let (tx, rx): (Sender<WorldUpdate>, Receiver<WorldUpdate>) = mpsc::channel();
	let mut server_connection_clone = server_connection.try_clone().unwrap();
	let network_manager =
		thread::spawn(move || network_connection::read_manager(&mut server_connection_clone, tx));

	let mut behaviour_queue: VecDeque<Behaviour> = VecDeque::new();

	//Initialize the player position and verify it to the server
	let position_and_look = rx.recv().expect("Connection failed");
	match position_and_look {
		WorldUpdate::PlayerUpdate(position_and_look) => {
			PLAYER.with_borrow_mut(|player| {
				player.x = position_and_look.x;
				player.y = position_and_look.y;
				player.z = position_and_look.z;
				player.yaw = position_and_look.yaw;
				player.pitch = position_and_look.pitch;
				player.on_ground = position_and_look.on_ground;
			});
		}
		_ => panic!("Should be a position and look"),
	}
	do_movement(Movements::NoInput(NoInput {}), &mut server_connection);

	loop {
		let keep_alive = Packets::KeepAlive(KeepAlive {});
		write_packet(&mut server_connection, keep_alive);

		if !behaviour_queue.is_empty() {
			let behaviour = behaviour_queue
				.pop_front()
				.expect("Should not be none because we just checked that it is some");
			do_action(behaviour.action, &mut server_connection);
			do_movement(behaviour.movement, &mut server_connection)
		} else {
			do_action(Actions::DoNothing(DoNothing {}), &mut server_connection);
			do_movement(Movements::NoInput(NoInput {}), &mut server_connection);
		}

		std::thread::sleep(time::Duration::from_millis(50));
	}
}
