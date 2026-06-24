use std::collections::HashMap;

use flate2::Status::Ok;

use crate::BLOCK_REGISTRY;
use crate::block::Coordinates;
use crate::registry::block_type::Collision;
use crate::world::World;
use crate::pathfinding::priority_queue::{PriorityQueue, Key};

//Uses D* lite with post processing to smooth paths
//References:
//https://idm-lab.org/bib/abstracts/papers/aaai02b.pdf
//http://www.cs.cmu.edu/~maxim/files/dlitemap_iros02.pdf
//http://www.cs.cmu.edu/~maxim/files/dlite_icra02.pdf

//stores the required data for D* lite to work with a node
struct Node {
	//on the path from this node to the goal, the next node/the previous node on the graph (because the graph orgiginates at the goal and works backwards)
	pub(super) previous: Coordinates,
	//one step look ahead based on g. see paper for more details
	pub(super) rhs: u32,
	//cost to goal. we know this because we know the rest of the path to the goal
	pub(super) g: u32,
}

//all coordinates refer to foot position unless specified otherwise
pub(crate) struct Path {
	//Starting position of player
	s_start: Coordinates,
	//Desired destination of player. This is where the seach starts (see D* lite for reasoning)
	s_goal: Coordinates,
	//Maps a coordinate to the next coordinate in the path from the first coordinate to the end coordinate, plus some other data D* lite requires. This is done because it is faster and more space efficient than storing a vec of nodes
	pub(super) nodes: HashMap<Coordinates, Node>,
	//priority queue
	U: PriorityQueue,
	//unsure what this does. TODO: figure out what it is
	k_m: u32,
}

//fns are implemented as defined in the D* lite paper
impl Path {
	fn calculate_key(&mut self, s: &Coordinates) -> Option<Key> {
		let node = self.nodes.get(s)?;

		return Some(
			Key {
				k_1: Ord::min(node.g, node.rhs + self.h(self.s_start, s)) + self.k_m,
				k_2: Ord::min(node.g, node.rhs),
			}
		);
	}

	//paper's description is confusing. will fill in as needed
	fn initialize(&mut self) {
	}

	fn update_vertex(&mut self, u: &Coordinates) -> Result {
		let node = self.nodes.get(u);
		let node_consistent = node.g == node.rhs;

		if node_consistent {
			self.U.remove(state);
		} else {
			self.U.insert_or_update(u, self.calculate_key(u).ok_or(Err(()))?);
		}

		return Ok(());
	}

	//outside behaviour should be the same as the function in the paper. internal mechanics differ slightly
	fn compute_shortest_path(&mut self) -> Result {
		while U.TopKey() < CalculateKey(s_start) || rhs(s_start) != g(s_start) {
			let (u, k_old) = self.U.pop()?;
			let k_new = self.calculate_key(u)?;
			
			let node_u = self.nodes.get(&u)?;
			if k_old < k_new {
				self.U.insert_or_update(&u, k_new);
			} else if node_u.g > node_u.rhs {
				node_u.g = node_u.rhs;
				for s in Path::pred(&u) {
					if s != self.s_goal {
						let node_s = self.nodes.get(&s)?;
						node_s.rhs = Ord::min(node_s.rhs, Path::c(&s, &u) + self.g(&u));
					}
					self.update_vertex(&s)?;
				}
			} else {
				let g_old = node_u.g;
				node_u.g = u32::MAX;
				let u_self_and_adjacent = Self::pred(&u);
				u_self_and_adjacent.push(u);
				for s in u_self_and_adjacent {
					let node_s = self.nodes.get(&s)?;
					if node_s.rhs == Path::c(&s, &u) + g_old || s == u {
						if s != self.s_goal {
							node_s.rhs = self.bellman(s, s_prime);
						}
					}
					self.update_vertex(&s)?;
				}
			}
		}

		return Ok(());
	}

	//called Main() in paper, but compute_path makes more sense
	fn compute_path(&mut self) {
		let mut s_last = self.s_start;
		self.initialize();
		self.compute_shortest_path();
		while(self.s_start != self.s_goal) {
			self.s_start = c(s_start, s_prime) + g(s_prime); //the value of this expression is the value of s_prime that minimizes the expression. s_prime is an element of succ(s_start) such that the value of this expression is minimized
			//move to s_start
			//scan graph for changed edge costs
			//if edge cost changed
				self.k_m = self.k_m + self.h(s_last, self.s_start);
				s_last = self.s_start;
				//for all directed edges (u, v) with changed edge costs
					let c_old = self.c(u, v);
					//update edge cost c(u, v);
					if c_old > self.c(u, v) {
						if u != self.s_goal {
							self.rhs(u) = Ord::min(self.rhs(u), self.c(u, v) + self.g(v));
						}
					}
					else if self.rhs(u) == c_old + self.g(v) {
						if u != self.s_goal {
							self.rhs(u) = Path::pred(s)
								.iter()
								.map(|s_prime| self.c(s, s_prime) + self.g(s_prime))
								.min();
						}
					}
					self.update_vertex(u);
			self.compute_shortest_path();
		}
	}

	//cost of moving from s to s_prime where s_prime is succ(s)
	fn c(s: &Coordinates, s_prime: &Coordinates) -> u32{
		//get the blocks at eye, foot, and below the player to see if breaking or placing is necessary
		let position_head = Coordinates {
			y: s_prime.y + 1,
			..*s_prime
		};
		let position_feet = Coordinates { ..*s_prime };
		let position_support = Coordinates {
			y: s_prime.y - 1,
			..*s_prime
		};

		let block_head = World::get_block(position_head);
		let block_feet = World::get_block(position_feet);
		let block_support = World::get_block(position_support);

		//very rough approximation of the costs
		//TODO: improve the cost prediction algorithm
		//head is free if it doesn't need to be removed, if not, it is the cost to remove
		let head_price = match block_head {
			Some(block_head) => {
				match (*BLOCK_REGISTRY)
					.get(&block_head.block_id)
					.expect("ID of block should be in BLOCK_REGISTRY")
					.collision
				{
					Collision::NonSolid => 1,
					Collision::Liquid => 3,
					Collision::Solid => 5,
				}
			}
			None => {
				//We don't know what the block is, so optimistically assume it is free
				0
			}
		};

		//feet are free if it doesn't need to be removed, if not, the cost is the cost to remove
		let feet_price = match block_feet {
			Some(block_feet) => {
				match (*BLOCK_REGISTRY)
					.get(&block_feet.block_id)
					.expect("ID of block should be in BLOCK_REGISTRY")
					.collision
				{
					Collision::NonSolid => 1,
					Collision::Liquid => 3,
					Collision::Solid => 5,
				}
			}
			None => {
				//We don't know what the block is, so optimistically assume it is free
				0
			}
		};

		//support block price is free if solid, if not, cost is cost to place block
		let support_price = match block_support {
			Some(block_support) => {
				match (*BLOCK_REGISTRY)
					.get(&block_support.block_id)
					.expect("ID of block should be in BLOCK_REGISTRY")
					.collision
				{
					Collision::NonSolid => 5,
					Collision::Liquid => 5,
					Collision::Solid => 0,
				}
			}
			None => {
				//We don't know what the block is, so optimistically assume it is free
				0
			}
		};

		return head_price + feet_price + support_price;
	}

	//not sure how to implement. returning the coordinates of nodes already in the graph may cause the algorithm to overflow and crash. if this happens, consider checking vertice presence in the list of nodes before adding it to the return vec
	//returns the predecessors of s on the graph
	fn pred(s: &Coordinates) -> Vec<Coordinates> {
		return vec![
			Coordinates {
				x: s.x + 1,
				..*s
			},
			Coordinates {
				x: s.x - 1,
				..*s
			},
			Coordinates {
				y: s.y + 1,
				..*s
			},
			Coordinates {
				y: s.y - 1,
				..*s
			},
			Coordinates {
				z: s.z + 1,
				..*s
			},
			Coordinates {
				z: s.z + 1,
				..*s
			},
		];
	}

	//just used same implementation of pred, not sure if this is right
	//returns the successors of s on the graph
	fn succ(s: &Coordinates) -> Vec<Coordinates> {
		return vec![
			Coordinates {
				x: s.x + 1,
				..*s
			},
			Coordinates {
				x: s.x - 1,
				..*s
			},
			Coordinates {
				y: s.y + 1,
				..*s
			},
			Coordinates {
				y: s.y - 1,
				..*s
			},
			Coordinates {
				z: s.z + 1,
				..*s
			},
			Coordinates {
				z: s.z + 1,
				..*s
			},
		];
	}

	//returns the value of g(s_prime) + c(s_prime, s) where s_prime is the pred(s) that produces the smallest value
	fn rhs(&self, s: &Coordinates) -> u32 {
		if s == self.s_start {
			return 0;
		} else {
			return Path::pred(s)
				.iter()
				.map(|s_prime| self.g(s_prime) + self.c(s_prime, s))
				.min();
		}
	}

	//returns the estimated cost to goal
	//this heuristic sucks, TODO: improve
	fn g(&self, s: &Coordinates) -> u32 {
		return self.s_goal.x.abs_diff(s.x) + self.s_goal.y.abs_diff(s.y) + self.s_goal.z.abs_diff(s.z);
	}
}
