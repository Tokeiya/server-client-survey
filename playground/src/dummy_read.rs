use super::loop_iterator::LoopIterator;
use std::iter::FusedIterator;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};
use tokio::time::{Duration, Instant, Sleep};

pub struct DummyRead<T: FusedIterator<Item = Duration>> {
	source: LoopIterator,
	duration: T,
	sleep: Pin<Box<Sleep>>,
}

impl<T: FusedIterator<Item = Duration>> DummyRead<T> {
	pub fn new(scr: LoopIterator, duration: T) -> Self {
		todo!()
	}
}

impl<T: FusedIterator<Item = Duration> + Unpin> AsyncRead for DummyRead<T> {
	fn poll_read(
		self: Pin<&mut Self>,
		cx: &mut Context<'_>,
		buf: &mut ReadBuf<'_>,
	) -> Poll<std::io::Result<()>> {
		let this = self.get_mut();

		let ret = if let Some(d) = this.duration.next() {
			let deadline = Instant::now() + d;
			this.sleep.as_mut().reset(deadline);
			this.sleep.as_mut().poll(cx)
		} else {
			Poll::Ready(())
		};

		match ret {
			Poll::Ready(_) => {
				if buf.remaining() != 0 {
					buf.filled_mut()[0] = this.source.next();
					buf.advance(1);
				}

				Poll::Ready(Ok(()))
			}
			Poll::Pending => Poll::Pending,
		}
	}
}
