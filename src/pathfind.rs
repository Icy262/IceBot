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
	fn calculate_key(s_start: &Coordinates, s: &Coordinates) -> u32 {
		return (
			Ord::min(g(s), rhs(s) + h(s_start, s)) + k_m,
			min(g(s), rhs(s)),
		);
	}
}
