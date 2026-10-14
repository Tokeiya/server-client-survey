use playground::dummy_read::*;
use std::iter::FusedIterator;
use tokio::io::AsyncReadExt;
use tokio::{
	runtime::Builder,
	select,
	time::{Duration, sleep},
};

struct Durations;

impl Iterator for Durations {
	type Item = Duration;

	fn next(&mut self) -> Option<Self::Item> {
		Some(Duration::from_millis(500))
	}
}

impl FusedIterator for Durations {}

async fn dummy(input: u64) -> u64 {
	sleep(Duration::from_micros(100)).await;
	input * 2
}

async fn worst_proc() {
	let mut dummy = DummyRead::new(LoopIterator::new((1..=255).collect()), Durations);
	let mut buff: [u8; 10] = [0; 10];

	for i in 0..10 {
		select! {
			result = dummy.read_exact(&mut buff) => {
				println!("{i} read done");
				println!("result:{:?}",result);

				println!("buff:{:?}",buff);
				buff.fill(0);

			}
			_ = sleep(Duration::from_secs(5)) => {
				println!("{i} timeout");
				println!("buff:{:?}",buff);
				buff.fill(0);
			}
		}

		println!();
	}
}

async fn proc() {
	let mut dummy = DummyRead::new(LoopIterator::new((1..=255).collect()), Durations);
	let mut buff: [u8; 10] = [0; 10];

	for i in 0..10 {
		let fut = dummy.read_exact(&mut buff);
		tokio::pin!(fut);

		loop {
			select! {
				result = &mut fut => {
					println!("{i} read done");
					println!("result:{:?}",result);

					println!("buff:{:?}",buff);
					buff.fill(0);

					break;

				}
				_ = sleep(Duration::from_secs(5)) => {
					println!("{i} timeout");
				}
			}
		}

		println!();
	}
}

fn main() {
	let runtime = Builder::new_multi_thread()
		.worker_threads(2)
		.enable_all()
		.build()
		.unwrap();

	runtime.block_on(proc());
}
