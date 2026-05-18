use std::{cell::RefCell, collections::LinkedList};
use std::net::TcpStream;
use std::thread;
use core::time;
use std::io::Write;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use crate::action_translator::{self, Actions};
use crate::block::Coordinates;
use crate::movement_translator::{self, Movements};
use crate::actions::{self, Join, Look};
use crate::movements::{self, Jump, NoInput, Walk};
use crate::packets::{KeepAlive, Packets, PlayerPositionandLook};
use crate::{packets::write_packet, player::Player};
use crate::network_connection;
use crate::world::WorldUpdate;
use crate::data_types::{MCBool, MCDouble, MCFloat};

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
	let join = Join {
		username: username,
	};
	let packets = Join::to_packets(join);
	for packet in packets {
		write_packet(&mut server_connection, packet);
	}

	//Start a network manager
	let (tx, rx): (Sender<WorldUpdate>, Receiver<WorldUpdate>) = mpsc::channel();
	let mut server_connection_clone = server_connection.try_clone().unwrap();
	let network_manager = thread::spawn(move || network_connection::read_manager(&mut server_connection_clone, tx));
	
	let mut action_queue: LinkedList<Actions> = LinkedList::new();
	let mut move_queue: LinkedList<Movements> = LinkedList::new();

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
		},
		_ => panic!("Should be a position and look"),
	}
	movements::do_action(Movements::NoInput(NoInput {}), &mut server_connection);

	loop {
		let keep_alive = Packets::KeepAlive(KeepAlive {});
		write_packet(&mut server_connection, keep_alive);

		if !action_queue.is_empty() {
			actions::do_action(action_queue.pop_front().expect("Should not be none because we just checked that it is some"), &mut server_connection);
		}

		if !move_queue.is_empty() {
			movements::do_action(move_queue.pop_front().expect("Should not be none because we just checked that it is some"), &mut server_connection)
		} else {
			movements::do_action(Movements::NoInput(NoInput {}), &mut server_connection);
		}

		std::thread::sleep(time::Duration::from_millis(50));
	}
}
