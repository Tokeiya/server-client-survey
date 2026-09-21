use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use tokio::sync::Notify;

pub struct Queue<T> {
	queue: VecDeque<T>,
	notify: Notify,
}

impl<T> Queue<T> {
	pub fn new() -> Self {
		Self {
			queue: VecDeque::new(),
			notify: Notify::new(),
		}
	}

	pub fn enqueue(&mut self, value: T) -> usize {
		self.queue.push_back(value);
		self.notify.notify_one();
		self.queue.len()
	}

	pub async fn dequeue(&mut self) -> T {
		loop {
			if let Some(value) = self.queue.pop_front() {
				return value;
			}
			self.notify.notified().await;
		}
	}
}
