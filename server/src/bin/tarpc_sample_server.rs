use futures::{FutureExt, Stream, StreamExt};
use server::Server;
use share_lib::{RemoteInterface, RemoteInterfaceRequest, RemoteInterfaceResponse};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use tarpc::context::Context;
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

	loop {
		select! {

			_=tokio::signal::ctrl_c()=>{
				println!("Shut down");
				break;
			}

			transport=listener.next().fuse()=>{
				let tran = transport.unwrap().unwrap();
				BaseChannel::with_defaults(tran)
					.execute(Server.serve())
					.for_each(|request| async move {
						select! {
						_=tokio::spawn(request)=>{}
						_=tokio::signal::ctrl_c().fuse()=>{
							println!("Shut down");
							return
						}
						}
					})
					.await;
			}

		}
	}

	println!("end server");
}

#[allow(dead_code)]

async fn alt_async_main() {
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

#[allow(dead_code)]
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

async fn loop_proc_alt<S, F>(mut channel: Pin<&mut S>)
where
	S: Stream<Item = F>,
	F: Future<Output = ()> + Send + 'static,
{
	while let Some(request) = channel.next().await {
		tokio::spawn(request);
	}
}

async fn flatten_async_alt_main() {
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

	'server: loop {
		cnt.store(0, Ordering::Relaxed);
		select! {
			_ = &mut token => {
				println!("Shut down");
				break 'server;
			}

			transport = listener.next() => {
				let tran = transport.unwrap().unwrap();
				let stream=BaseChannel::with_defaults(tran).execute(Server.serve());
				pin!(stream);
				select! {
					_=&mut token => {
						println!("Shut down");
						break 'server;
					}

					_=loop_proc_alt(stream)=>{
						println!("Completed");
					}
				}


			}
		}
	}
}

#[allow(dead_code)]
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

	'server: loop {
		cnt.store(0, Ordering::Relaxed);
		select! {
			_ = &mut token => {
				println!("Shut down");
				break 'server;
			}

			transport = listener.next() => {
				let tran = transport.unwrap().unwrap();
				let stream=BaseChannel::with_defaults(tran).execute(Server.serve());
				select! {
					_=&mut token => {
						println!("Shut down");
						break 'server;
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
		.block_on(flatten_async_alt_main());
}
