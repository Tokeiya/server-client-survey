use tokio::time::{sleep,Sleep,Instant};
use tokio::runtime::{Runtime,Builder};
use std::future::{Future};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

#[derive(Default)]
struct Envelope(Option<Pin<Box<Sleep>>>);



impl Future for Envelope {
	type Output = ();
	
	fn poll(
		self: Pin<&mut Self>,
		cx: &mut Context<'_>,
	) -> Poll<Self::Output> {
		println!("enter poll");
		
		let this = self.get_mut();
		
		if this.0.is_none() {
			println!("assign sleep");
			this.0 = Some(Box::pin(
				sleep(Duration::from_secs(1))
			));
		}
		
		match this.0.as_mut().unwrap().as_mut().poll(cx) {
			Poll::Ready(()) =>{
				println!("exit:ready");
				let deadline=Instant::now()+Duration::from_secs(1);
				this.0.as_mut().unwrap().as_mut().reset(deadline);
				Poll::Ready(()) },
			Poll::Pending => {println!("exit:pending");
				Poll::Pending},
		}
	}
}


fn main() {
	let rt=Builder::new_current_thread().enable_time().build().unwrap();
	
	rt.block_on(async{
		let mut envelope=Envelope::default();
		
		for i in 0..5{
			println!("enter await:{i}");
			(&mut envelope).await;
			println!("exit await:{i}\n\n");
		}
	})
}