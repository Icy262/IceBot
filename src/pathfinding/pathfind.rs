use std::collections::HashMap;

use crate::BLOCK_REGISTRY;
use crate::pathfinding::priority_queue::{Key, PriorityQueue};
use crate::registry::block_type::Collision;
use crate::world::block::Coordinates;
use crate::world::world::World;

//Uses D* lite with post processing to smooth paths
//References:
//https://idm-lab.org/bib/abstracts/papers/aaai02b.pdf
//http://www.cs.cmu.edu/~maxim/files/dlitemap_iros02.pdf
//http://www.cs.cmu.edu/~maxim/files/dlite_icra02.pdf

//stores the required data for D* lite to work with a node
#[derive(Clone, Copy)]
struct Node {
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
	//Maps a coordinate to the associated node, which contains the rhs and g values. This is done because it is faster and more space efficient than storing a vec of all the nodes
	nodes: HashMap<Coordinates, Node>,
	//priority queue
	U: PriorityQueue,
	k_m: u32,
}

//fns are implemented as defined in the D* lite paper
impl Path {
	//TODO: implement remainder, per paper
	pub(crate) fn new(s_start: &Coordinates, s_goal: &Coordinates) -> Self {
		let mut path = Self {
			s_start: *s_start,
			s_goal: *s_goal,
			nodes: HashMap::new(),
			U: PriorityQueue::new(),
			k_m: 0,
		};
		
		path.nodes.insert(
			*s_start,
			Node {
				g: u32::MAX,
				rhs: u32::MAX,
			},
		);

		path.nodes.insert(
			*s_goal,
			Node {
				g: u32::MAX,
				rhs: 0,
			},
		);

		let goal = &path.calculate_key(s_goal).expect("goal node cannot be undefined because we just inserted it");
		path.U.insert_or_update(s_goal, goal);
		
		return path;
	}

	pub(crate) fn update_position(&mut self, new_s_start: &Coordinates) {
		self.s_start = *new_s_start;
		self.nodes.insert(
			*new_s_start,
			Node {
				g: u32::MAX,
				rhs: u32::MAX,
			}
		);
	}

	//will return the next node in the path from the position passed to the goal. will return None if this node does not exist
	pub(crate) fn trace_path(&mut self, position: &Coordinates) -> Option<Coordinates> {
		self.compute_shortest_path()?;
		return self.best_successor(position).ok();
	}

	//TODO: implement updating edge costs

	fn calculate_key(&mut self, s: &Coordinates) -> Option<Key> {
		let node = self.nodes.get(s)?;

		return Some(Key {
			k_1: Ord::min(node.g, node.rhs + self.h(&self.s_start, s)) + self.k_m,
			k_2: Ord::min(node.g, node.rhs),
		});
	}

	fn update_vertex(&mut self, u: &Coordinates) -> Result<(), &'static str> {
		let node = self.nodes.get(u).ok_or("u not found")?;
		let node_consistent = node.g == node.rhs;

		if node_consistent {
			self.U.remove(u);
		} else {
			let key = self.calculate_key(u).ok_or("insert or update failed")?;
			self.U.insert_or_update(u, &key);
		}

		return Ok(());
	}

	//outside behaviour should be the same as the function in the paper. internal mechanics differ slightly
	fn compute_shortest_path(&mut self) -> Result<(), &'static str> {
		let s_start_node = (*self
			.nodes
			.get(&self.s_start)
			.ok_or("start node not defined")?)
		.clone();
		let s_start_key = self
			.calculate_key(&(self.s_start.clone()))
			.ok_or("start node not defined")?;
		while self.U.peek().ok_or("no solution exists")?.1 < s_start_key
			|| s_start_node.rhs != s_start_node.g
		{
			let (u, k_old) = self.U.pop().ok_or("no solution exists")?;
			let k_new = self.calculate_key(&u).ok_or("could not calculate key")?;

			let node_u = self.nodes.get_mut(&u).ok_or("could not find node")?;
			if k_old < k_new {
				self.U.insert_or_update(&u, &k_new);
			} else if node_u.g > node_u.rhs {
				node_u.g = node_u.rhs;
				for s in Path::pred(&u) {
					if s != self.s_goal {
						let g = self.nodes.get(&u).ok_or("could not find node")?.g;
						let node_s = self.nodes.get_mut(&s).ok_or("could not find node")?;
						node_s.rhs = Ord::min(node_s.rhs, Path::c(&s, &u) + g);
					}
					self.update_vertex(&s)?;
				}
			} else {
				let g_old = node_u.g;
				node_u.g = u32::MAX;
				let mut u_self_and_adjacent = Self::pred(&u);
				u_self_and_adjacent.push(u);
				for s in u_self_and_adjacent {
					let bellman = self.bellman(&s)?;
					let node_s = self.nodes.get_mut(&s).ok_or("could not find node")?;
					if node_s.rhs == Path::c(&s, &u) + g_old || s == u {
						if s != self.s_goal {
							node_s.rhs = bellman;
						}
					}
					self.update_vertex(&s)?;
				}
			}
		}

		return Ok(());
	}

	//cost of moving from s to s_prime where s_prime is succ(s)
	fn c(s: &Coordinates, s_prime: &Coordinates) -> u32 {
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
			Coordinates { x: s.x + 1, ..*s },
			Coordinates { x: s.x - 1, ..*s },
			Coordinates { y: s.y + 1, ..*s },
			Coordinates { y: s.y - 1, ..*s },
			Coordinates { z: s.z + 1, ..*s },
			Coordinates { z: s.z + 1, ..*s },
		];
	}

	//just used same implementation of pred, not sure if this is right
	//returns the successors of s on the graph
	fn succ(s: &Coordinates) -> Vec<Coordinates> {
		return vec![
			Coordinates { x: s.x + 1, ..*s },
			Coordinates { x: s.x - 1, ..*s },
			Coordinates { y: s.y + 1, ..*s },
			Coordinates { y: s.y - 1, ..*s },
			Coordinates { z: s.z + 1, ..*s },
			Coordinates { z: s.z + 1, ..*s },
		];
	}

	//returns the value of g(s_prime) + c(s_prime, s) where s_prime is the pred(s) that produces the smallest value
	fn bellman(&self, s: &Coordinates) -> Result<u32, &'static str> {
		if *s == self.s_start {
			return Ok(0);
		} else {
			let mut result = u32::MAX;
			for s_prime in Path::pred(s) {
				result = Ord::min(
					result,
					self.nodes.get(&s_prime).ok_or("could not find node")?.g + Path::c(&s_prime, s),
				);
			}
			return Ok(result);
		}
	}

	//returns the node for s_prime that would produce the lowest g(s_prime) + c(s_prime, s) where s_prime is the pred(s)
	fn best_successor(&self, s: &Coordinates) -> Result<Coordinates, &'static str> {
		let mut best = None;
		let mut best_cost = u32::MAX;

		for succ in Path::succ(s) {
			let node = self.nodes.get(&succ).ok_or("could not find node")?;
			let cost = node.g + Path::c(s, &succ);

			if cost < best_cost {
				best_cost = cost;
				best = Some(succ);
			}
		}

		best.ok_or("no path exists")
	}

	//returns the estimated cost between two points
	//this heuristic sucks, TODO: improve
	fn h(&self, s: &Coordinates, s_prime: &Coordinates) -> u32 {
		return s.x.abs_diff(s_prime.x) + s.y.abs_diff(s_prime.y) + s.z.abs_diff(s_prime.z);
	}
}
