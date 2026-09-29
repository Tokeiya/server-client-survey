use futures::{FutureExt, Stream, StreamExt};
use server::Server;
use share_lib::{RemoteInterface, RemoteInterfaceRequest, RemoteInterfaceResponse};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use tarpc::serde_transport::unix::Incoming;
use tarpc::{
	ClientMessage, Response,
	serde_transport::unix,
	server::{BaseChannel, Channel},
	tokio_serde::formats::Bincode,
};
use tokio::{pin, select};

async fn loop_proc<S, F>(channel: S)
where
	S: Stream<Item = F>,
	F: Future<Output = ()> + Send + 'static,
{
	tokio::pin!(channel);

	while let Some(request) = channel.next().await {
		println!("copy");
		tokio::spawn(request);
	}
}

async fn flatten_async_main() {
	println!("flatten start server");

	let path = unix::TempPathBuf::new("/tmp/sock");
	let mut listener: Incoming<
		ClientMessage<RemoteInterfaceRequest>,
		Response<RemoteInterfaceResponse>,
		Bincode<_, _>,
		_,
	> = unix::listen(&path, Bincode::default).await.unwrap();

	let token = tokio::signal::ctrl_c();
	pin!(token);
	let cnt = Arc::new(AtomicUsize::new(0));

	loop {
		cnt.store(0, Ordering::Relaxed);
		select! {
			_ = &mut token => {
				println!("Shut down");
				break;
			}

			transport = listener.next() => {
				let tran = transport.unwrap().unwrap();
				let stream=BaseChannel::with_defaults(tran).execute(Server.serve());
				select! {
					_=&mut token => {
						println!("Shut down");
						break;
					}

					_=loop_proc(stream)=>{
						println!("Completed");
					}
				}
			}
		}
	}
}

fn main() {
	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(flatten_async_main());
}
