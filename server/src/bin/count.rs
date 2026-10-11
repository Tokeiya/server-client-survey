use std::{
	future::Future,
	pin::Pin,
	sync::{Arc,Mutex},
	task::{Context,Poll,Waker},
	sync::atomic::{AtomicUsize,Ordering}
};
use std::sync::atomic::AtomicBool;

#[derive(Default)]
struct State{
	awake_count:AtomicUsize,
	poll_count:AtomicUsize,
	is_ready:AtomicBool,
	waker:Mutex<Option<Waker>>
}


struct Result{
	pub poll_count:usize,
	pub awake_count:usize
}

struct Awaiter(Arc<State>);
struct Handle(Arc<State>);

impl Handle{
	fn awake(&self){
		
		if let Some(waker)=self.0.waker.lock().unwrap().as_ref(){
			self.0.awake_count.fetch_add(1,Ordering::SeqCst);
			waker.wake_by_ref();
		}
	}
	
	fn set_ready(&self){
		self.0.is_ready.store(true,Ordering::SeqCst);
		if let Some(waker)=self.0.waker.lock().unwrap().take(){
			waker.wake();
		}
	}
}

fn new() -> (Awaiter,Handle){
	let state = Arc::new(State::default());
	(Awaiter(state.clone()),Handle(state))
}

impl Future for Awaiter {
	type Output = Result;
	
	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		_=self.0.waker.lock().unwrap().replace(cx.waker().clone());
		
		self.0.poll_count.fetch_add(1, Ordering::SeqCst);
		if self.0.is_ready.load(Ordering::SeqCst) {
			Poll::Ready(Result {
				poll_count: self.0.poll_count.load(Ordering::SeqCst),
				awake_count: self.0.awake_count.load(Ordering::SeqCst)
			})
		} else {
			Poll::Pending
		}
	}
}

fn main() {
	let rt=tokio::runtime::Builder::new_current_thread()
		.build().unwrap();
	
	let (awaiter,handle)=new();
	
	std::thread::spawn(move||{
		
		loop {
			println!("input ready...");
			let mut buff=String::default();
			std::io::stdin().read_line(&mut buff).unwrap();
			
			
			if buff == "awake\n" {
				for _ in 0..100 {
					handle.awake();
				}
			} else if buff == "ready\n" {
				handle.set_ready();
			} else {
				println!("{} unknown command", buff);
			}
		}
	});
	
	rt.block_on(async {
		let result=awaiter.await;
		println!("poll_count={}, awake_count={}",result.poll_count,result.awake_count);
	});
	
	
}