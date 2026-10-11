use std::{
	future::Future,
	pin::Pin,
	sync::{Arc, Mutex},
	task::{Context, Poll, Waker},
};

#[derive(Default)]
struct State {
	count: usize,
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

		if state.waker.is_none() {
			state.waker = Some(cx.waker().clone());
		}

		if state.count >= 3 {
			println!("poll -> Ready (3回達成)");
			Poll::Ready(())
		} else {
			println!("poll -> Pending (現在 {} 回)", state.count);
			Poll::Pending
		}
	}
}

impl Handle {
	fn awake(&mut self) {
		let mut state = self.0.lock().unwrap();

		if state.waker.is_some() {
			state.count += 1;

			let waker = state.waker.as_mut().unwrap();
			println!("wake by ref!");
			waker.wake_by_ref();
		}
	}
}

fn main() {
	let rt = tokio::runtime::Builder::new_current_thread()
		.build()
		.unwrap();

	let (manual, mut handle) = Manual::new();

	std::thread::spawn(move || {
		for _ in 0..3 {
			println!("press enter to increment");
			std::io::stdin().read_line(&mut String::new()).unwrap();
			handle.awake();
		}
	});

	rt.block_on(async {
		manual.await;
		println!("completed!");
	});
}
