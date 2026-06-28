use crate::tasks::tasks::Tasks;

pub(super) struct PrioritisedTask {
	pub(super) task: Tasks,
	pub(super) priority_function: Box<dyn FnMut() -> usize>,
}
