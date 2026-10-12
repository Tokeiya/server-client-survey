use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};
use tokio::time::{Sleep,sleep};

struct DummyLoad;

impl AsyncRead for DummyLoad {
	fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
		todo!()
	}
}

fn main() {
	
}