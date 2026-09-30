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

#[allow(dead_code)]
async fn async_main() {
	println!("start server");

	let path = unix::TempPathBuf::new("/tmp/sock");
	let mut listener = unix::listen(&path, Bincode::default).await.unwrap();

	let token = tokio::signal::ctrl_c();
	pin!(token);
	let cnt = Arc::new(AtomicUsize::new(0));

	'server: loop {
		println!("server loop start");
		cnt.store(0, Ordering::Relaxed);
		select! {
			_ = &mut token => {
				println!("Shut down");
				break 'server;
			}

			transport = listener.next() => {
				println!("new connection");
				let tran = transport.unwrap().unwrap();


				select! {
					_ = &mut token => {
						println!("Shut down");
						break 'server;
					}

					_ = BaseChannel::with_defaults(tran)
						.execute(Server.serve())
						.for_each(async |request| {
						println!("copy");
							let value=cnt.fetch_add(1, Ordering::Relaxed);
							println!("Processing request {:?}",value);
							tokio::spawn(request);}) => {
						println!("connection closed");
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
		.block_on(async_main());
}
