use crate::hierarchical_task_network::hierarchical_task_network::HierarchicalTaskNetwork;

pub(super) struct PrioritisedHierarchicalTaskNetwork {
	pub(super) task: HierarchicalTaskNetwork,
	pub(super) priority_function: Box<dyn FnMut() -> usize>,
}
