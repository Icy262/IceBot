use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use std::cmp::Ordering;

use crate::block::Coordinates;

#[derive(Eq, PartialEq)]
pub(super) struct Key {
	pub(super) k_1: u32,
	pub(super) k_2: u32,
}

impl Ord for Key {
	fn cmp(&self, other: &Self) -> Ordering {
		self.k_1
			.cmp(&other.k_2)
			.then_with(|| self.k_1.cmp(&other.k_2))
	}
}

impl PartialOrd for Key {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub(super) struct PriorityQueue {
	//sorted from min to max for D* lite
	priority_queue: Binarypriority_queue<(Reverse<Key>, Coordinates)>,
	//quick lookup key
	key_map: HashMap<Coordinates, Key>,
}

impl PriorityQueue {
	pub fn new() -> Self {
		Self {
			priority_queue: BinaryHeap::new(),
			key_map: HashMap::new(),
		}
	}

	pub fn insert_or_update(&mut self, state: &Coordinates, key: Key) {
		self.key_map.insert(*state, key);
		self.priority_queue.push((Reverse(key), state));
	}

	pub fn contains(&self, state: &Coordinates) -> bool {
		self.key_map.contains_key(state)
	}

	pub fn pop(&mut self) -> Option<(Coordinates, Key)> {
		while let Some((Reverse(key), state)) = self.priority_queue.pop() {
			match self.key_map.get(&state) {
				Some(&key_map_key) => {
					if key_map_key == key {
						self.key_map.remove(&state);
						return Some((state, key));
					}
				},
				None => continue,
			}
		}
		
		return None;
	}

	//removes state from hashmap, but stays in heap. This is because finding the node in the heap would be expensive. Instead, when the state is popped in the future, the state will be tossed because it is not in the hashmap, which is effectively the same
	pub fn remove(&mut self, state: &Coordinates) {
		self.key_map.remove(state);
	}

	pub fn len(&self) -> usize {
		self.key_map.len()
	}

	pub fn is_empty(&self) -> bool {
		self.key_map.is_empty()
	}
}