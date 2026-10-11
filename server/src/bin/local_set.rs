use std::{
	future::Future,
	pin::Pin,
	sync::{Arc, Mutex},
	task::{Context, Poll, Waker},
	time::Duration,
};

use std::sync::OnceLock;

#[derive(Default)]
struct State {
	ready: bool,
	waker: Option<Waker>,
}

struct Manual(Arc<Mutex<State>>);
struct Handle(Arc<Mutex<State>>);

impl Manual {
	fn new() -> (Self, Handle) {
		let state = Arc::new(Mutex::new(State::default()));
		(Self(state.clone()), Handle(state))
	}
}

impl Future for Manual {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
		let mut state = self.0.lock().unwrap();

		if state.ready {
			println!("poll -> Ready");
			Poll::Ready(())
		} else {
			println!("poll -> Pending");
			state.waker = Some(cx.waker().clone());
			Poll::Pending
		}
	}
}

impl Handle {
	fn awake(&self) {
		let waker = {
			let mut state = self.0.lock().unwrap();
			state.ready = true;
			state.waker.take()
		};

		if let Some(waker) = waker {
			println!("wake!");
			waker.wake();
		}
	}
}

fn main() {
	let rt = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();

	let (manual, handle) = Manual::new();

	std::thread::spawn(move || {
		println!("press enter to continue");
		std::io::stdin().read_line(&mut String::new()).unwrap();
		handle.awake();
	});

	rt.block_on(async {
		println!("enter await");
		manual.await;
		println!("completed!");
	});
}
