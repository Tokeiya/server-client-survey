use tokio::{
	runtime::{Builder, LocalOptions, LocalRuntime, Runtime},
	select,
	sync::mpsc::{Receiver, Sender},
	sync::oneshot,
	task::{JoinError, JoinHandle},
	time::{Duration, sleep},
};

async fn dummy(input: u64) -> u64 {
	sleep(Duration::from_micros(100)).await;
	input * 2
}

fn main() {
	let runtime = Builder::new_multi_thread()
		.worker_threads(2)
		.enable_all()
		.build()
		.unwrap();

	let a = runtime.block_on(futures::future::join3(dummy(10), dummy(20), dummy(30)));

	println!("{:?}", a);
}
