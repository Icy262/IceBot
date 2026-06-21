use std::cmp::min_by;
use std::collections::HashMap;

use crate::BLOCK_REGISTRY;
use crate::block::Coordinates;
use crate::registry::block_type::Collision;
use crate::world::World;

//Uses D* lite with post processing to smooth paths
//References:
//https://idm-lab.org/bib/abstracts/papers/aaai02b.pdf
//http://www.cs.cmu.edu/~maxim/files/dlitemap_iros02.pdf
//http://www.cs.cmu.edu/~maxim/files/dlite_icra02.pdf

//all coordinates refer to foot position unless specified otherwise
pub(crate) struct Path {
	//Starting position of player
	s_start: Coordinates,
	//Desired destination of player. This is where the seach starts (see D* lite for reasoning)
	s_goal: Coordinates,
	//Maps a coordinate to the next coordinate in the path from the first coordinate to the end coordinate and the price to get to that coordinate from the end coordinate. This is done because it is faster and more space efficient than storing a vec of nodes
	nodes: HashMap<Coordinates, (u32, Coordinates)>,
}

//fns are implemented as defined in the D* lite paper
impl Path {
	fn calculate_key(&mut self, s_start: &Coordinates, s: &Coordinates) -> u32 {
		return (
			Ord::min(g(s), rhs(s) + h(s_start, s)) + k_m,
			min(g(s), rhs(s)),
		);
	}

	//paper's description is confusing. will fill in as needed
	fn initialize(&mut self) {
	}

	fn update_vertex(&mut self, u: &Coordinates) {
		if u != s_goal {
			rhs(u) = Org::min(c(u, s_prime) + g(s_prime));
		}

		if u in U {
			U.remove(u);
		}

		if g(u) != rhs(u) {
			U.insert(u, calculate_key(u));
		}
	}

	fn compute_shortest_path(&mut self) {
		while U.top_key() < calculate_key(s_start) || rhs(s_start) != g(s_start) {
			k_old = U.top_key();
			u = U.pop();
			if(k_old < calculate_key(u)) {
				U.insert(u, calculate_key(u));
			} else if g(u) > rhs(u) {
				g(u) = rhs(u);
				for s in pred(u) {
					update_vertex(s);
				}
			} else {
				g(u) = u32::MAX;
				for s in (pred(u) || u) {
					update_vertex(s);
				}
			}
		}
	}

	//called Main() in paper, but compute_path makes more sense
	fn compute_path(&mut self) {
		let s_last = self.s_start;
		self.initialize();
		self.compute_path();
		while(s_start != s_goal) {
			s_start = c(s_start, s_prime) + g(s_prime); //s_prime is an element of succ(s_start) such that the value of this expression is minimized
		//scan for changed edge costs
		//if edge cost changed
			k_m = k_m + h(s_last, s_start);
			s_last = s_start;
			//for directed edge (u, v) with changed edge costs
				//update edge cost c(u, v);
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
}
