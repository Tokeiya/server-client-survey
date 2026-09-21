use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::{
	runtime::{Builder, Runtime},
	select,
	sync::mpsc::{Receiver, Sender},
	sync::oneshot,
	task::{JoinError, JoinHandle},
	time::{Duration, sleep},
};

pub enum Command {
	Value(String),
	Dump(Sender<usize>),
}

pub struct Executor {
	runtime: Runtime,
	task: JoinHandle<usize>,
}

impl Executor {
	pub fn new(rx: Receiver<Command>) -> Self {
		let runtime = Builder::new_multi_thread()
			.worker_threads(1)
			.enable_all()
			.build()
			.unwrap();

		let task = runtime.spawn(Self::loop_proc(rx));

		Self { runtime, task }
	}

	async fn proc(queue: &mut VecDeque<(usize, String)>) {
		todo!()
	}

	async fn receive_proc(
		rx: &mut Receiver<Command>,
		queue: VecDeque<(usize, String)>,
		seed: usize,
	) -> Result<usize, JoinError> {
		todo!()
	}

	async fn loop_proc(mut rx: Receiver<Command>) -> usize {
		let mut queue: VecDeque<(usize, String)> = VecDeque::new();
		let mut cnt = 0usize;

		loop {
			select! {
				biased;

				cmd = Self::receive_proc(&mut rx, queue, cnt) => {
					todo!()
				}

			}
			todo!()
		}
	}

	fn join(self) -> Result<usize, JoinError> {
		let Self { runtime, task } = self;
		runtime.block_on(task)
	}
}
