use core::time;
use std::collections::VecDeque;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::{cell::RefCell, collections::LinkedList};

use crate::behaviour::actions::{Actions, DoNothing, Join, do_action};
use crate::behaviour::behaviour::Behaviour;
use crate::behaviour::movements::to_packets;
use crate::behaviour::movements::{Jump, Movements, NoInput, Walk, do_movement};
use crate::hierarchical_task_network::hierarchical_task_network::HierarchicalTaskNetwork;
use crate::network::data_types::{MCBool, MCDouble, MCFloat};
use crate::network::data_types::{MCMetadata, MCUByte};
use crate::network::network_connection;
use crate::network::packets::write_packet;
use crate::network::packets::{KeepAlive, Packets, PlayerPositionandLook};
use crate::player::Player;
use crate::scheduler::scheduler::Schedule;
use crate::tasks::go_to::GoTo;
use crate::world::block::{self, Block, Coordinates};
use crate::world::entity::Position;
use crate::world::world::{WORLD_MODEL, World, WorldUpdate};
use crate::{BLOCK_REGISTRY, behaviour, world};

thread_local! {
	pub(crate) static PLAYER: RefCell<Player> = RefCell::new(
		Player {
			position: Position {
				x: 0f64,
				y: 0f64,
				z: 0f64,
			},
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

	//Initialize the player position and verify it to the server
	let position_and_look = rx.recv().expect("Connection failed");
	match position_and_look {
		WorldUpdate::PlayerUpdate(position_and_look) => {
			PLAYER.with_borrow_mut(|player| {
				player.position = Position {
					x: position_and_look.position.x,
					y: position_and_look.position.y,
					z: position_and_look.position.z,
				};
				player.yaw = position_and_look.yaw;
				player.pitch = position_and_look.pitch;
				player.on_ground = position_and_look.on_ground;
			});
		}
		_ => panic!("Should be a position and look"),
	}
	do_movement(Movements::NoInput(NoInput {}), &mut server_connection);

	let mut task_scheduler = Schedule::new();

	//for testing only
	let walk_task = HierarchicalTaskNetwork::new(crate::tasks::tasks::Tasks::GoTo(GoTo::new(
		&PLAYER.with_borrow(|&player| {
			return Coordinates {
				x: player.position.x.floor() as i32 - 5,
				y: player.position.y.floor() as i32 ,
				z: player.position.z.floor() as i32  - 3,
			};
		}),
	)));
	task_scheduler.push_task_network(walk_task, Box::new(|| return 1));

	let mut count = 0;
	loop {
		let keep_alive = Packets::KeepAlive(KeepAlive {});
		write_packet(&mut server_connection, keep_alive);

		if let Ok(msg) = rx.try_recv() {
			match msg {
				WorldUpdate::PlayerUpdate(position_and_look) => {
					println!("Recieved position update from server");
					PLAYER.with_borrow_mut(|player| {
						player.position = Position {
							x: position_and_look.position.x,
							y: position_and_look.position.y,
							z: position_and_look.position.z,
						};
						player.yaw = position_and_look.yaw;
						player.pitch = position_and_look.pitch;
						player.on_ground = position_and_look.on_ground;
						player.vx = 0.0;
						player.vy = 0.0;
						player.vz = 0.0;
					});

					let packets = to_packets(Movements::NoInput(NoInput {})).remove(0);
					write_packet(&mut server_connection, packets);

					continue;
				}
				_ => panic!("Should be a position and look"),
			}
		}

		let Some(behaviour) = task_scheduler.get_next_behaviour() else {
			println!("skipped");
			continue;
		};
		do_action(behaviour.action, &mut server_connection);
		do_movement(behaviour.movement, &mut server_connection);

		std::thread::sleep(time::Duration::from_millis(50));
	}
}
