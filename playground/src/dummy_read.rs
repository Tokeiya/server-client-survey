use super::loop_iterator::LoopIterator;
use std::iter::FusedIterator;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};
use tokio::time::{Duration, Instant, Sleep, sleep};

pub struct DummyRead<T: FusedIterator<Item = Duration>> {
	source: LoopIterator,
	duration: T,
	sleep: Option<Pin<Box<Sleep>>>,
}

impl<T: FusedIterator<Item = Duration>> DummyRead<T> {
	pub fn new(scr: LoopIterator, duration: T) -> Self {
		Self {
			source: scr,
			duration,
			sleep: Some(Box::pin(sleep(Duration::from_millis(0)))),
		}
	}
}

impl<T: FusedIterator<Item = Duration> + Unpin> AsyncRead for DummyRead<T> {
	fn poll_read(
		self: Pin<&mut Self>,
		cx: &mut Context<'_>,
		buf: &mut ReadBuf<'_>,
	) -> Poll<std::io::Result<()>> {
		let mut this = self.get_mut();
		let mut flg = false;

		match this.sleep.as_mut() {
			Some(slp) => match slp.as_mut().poll(cx) {
				Poll::Ready(_) => {
					todo!()
				}
				Poll::Pending => Poll::Pending,
			},
			None => {
				let datum: [u8; 1] = [this.source.next()];
				buf.put_slice(&datum);
				Poll::Ready(Ok(()))
			}
		}
	}
}
