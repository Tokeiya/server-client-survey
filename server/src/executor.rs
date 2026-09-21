use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::{
	runtime::{Builder, LocalOptions, LocalRuntime, Runtime},
	select,
	sync::mpsc::{Receiver, Sender},
	sync::oneshot,
	task::{JoinError, JoinHandle},
	time::{Duration, sleep},
};

use super::queue::Queue;

pub enum Command {
	Value(String),
	Dump(Sender<usize>),
}

pub struct Executor {
	runtime: LocalRuntime,
	task: JoinHandle<usize>,
}

impl Executor {
	pub fn new(rx: Receiver<Command>) -> Self {
		let runtime = Builder::new_current_thread()
			.enable_all()
			.build_local(LocalOptions::default())
			.unwrap();

		let task = runtime.spawn_local(Self::loop_proc(rx));

		Self { runtime, task }
	}

	async fn proc(queue: &mut Queue<(usize, String)>) {
		todo!()
	}

	async fn receive_proc(
		rx: &mut Receiver<Command>,
		queue: &Queue<(usize, String)>,
		seed: usize,
	) -> Result<usize, JoinError> {
		todo!()
	}

	async fn loop_proc(mut rx: Receiver<Command>) -> usize {
		let mut queue: Queue<(usize, String)> = Queue::new();
		let mut cnt = 0usize;

		loop {
			select! {
				biased;


			}
			todo!()
		}
	}

	fn join(self) -> Result<usize, JoinError> {
		let Self { runtime, task } = self;
		runtime.block_on(task)
	}
}
