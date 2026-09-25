use anyhow::Result as AnyResult;
use futures::{FutureExt, poll};
use std::task::Poll;
use tokio::runtime::{Builder, Runtime};
use tokio::time::{Duration, sleep};

fn main() {
	let runtime = Builder::new_multi_thread()
		.worker_threads(1)
		.enable_all()
		.build()
		.unwrap();

	let a = runtime.block_on(process_loop());

	_ = dbg!(a);
}

async fn process_loop() -> AnyResult<()> {
	println!("enter");
	let mut cnt = 0u64;
	let mut signal = Box::pin(tokio::signal::ctrl_c());

	loop {
		match poll!(&mut signal) {
			Poll::Ready(_) => {
				println!("Ctrl-C detected");
				break AnyResult::Ok(());
			}
			Poll::Pending => {}
		}
		println!("loop:{cnt}");
		cnt = cnt.wrapping_add(1);
		sleep(Duration::from_millis(100)).await;
	}
}
