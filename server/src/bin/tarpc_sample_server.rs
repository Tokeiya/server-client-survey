use futures::stream::Next;
use futures::{FutureExt, StreamExt};
use share_lib::{
	RemoteInterface, RemoteInterfaceClient, RemoteInterfaceRequest, RemoteInterfaceResponse,
};
use std::io::Error;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use tarpc::context::Context;
use tarpc::serde_transport::Transport;
use tarpc::serde_transport::unix::Incoming;
use tarpc::server::TrackedRequest;
use tarpc::{
	ChannelError, ClientMessage, Response,
	serde_transport::unix,
	server::{BaseChannel, Channel},
	tokio_serde::formats::Bincode,
};
use tokio::net::UnixStream;
use tokio::{pin, select};

#[derive(Debug, Clone)]
struct Server;

impl RemoteInterface for Server {
	async fn say_hello(self, context: Context, name: String) -> String {
		println!("Hello {name}");
		format!("Hello, {}!", name)
	}
}

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

async fn loop_proc(
	mut channel: BaseChannel<
		RemoteInterfaceRequest,
		RemoteInterfaceResponse,
		Transport<
			UnixStream,
			ClientMessage<RemoteInterfaceRequest>,
			Response<RemoteInterfaceResponse>,
			Bincode<ClientMessage<RemoteInterfaceRequest>, Response<RemoteInterfaceResponse>>,
		>,
	>,
	token: &mut Pin<&mut impl Future>,
) -> bool {
	loop {
		select! {
			_=token=>{
				return false
			}

			item=channel.next()=>{
				match item{

				None => {return false}
					Some(hoge) => {
						match hoge{

						Ok(piyo) => {
								prin
							}
							Err(_) => {}}

					}}

			}
		}
	}
}

async fn flatten_async_main() {
	println!("start server");

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
				let mut channel=BaseChannel::with_defaults(tran);


			}
		}
	}
}

fn main() {
	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(alt_async_main());
}
