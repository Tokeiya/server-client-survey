use futures::{FutureExt, StreamExt};
use share_lib::{RemoteInterface, RemoteInterfaceClient};
use tarpc::context::Context;
use tarpc::{
	client, context,
	serde_transport::unix,
	server::{BaseChannel, Channel},
	tokio_serde::formats::Bincode,
};
use tokio::select;

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
	let mut listener = unix::listen(path, Bincode::default).await.unwrap();

	loop {
		futures::select! {

			_=tokio::signal::ctrl_c().fuse()=>{
				println!("Shut down");
				break;
			}


			transport=listener.next().fuse()=>{
				let tran = transport.unwrap().unwrap();
				BaseChannel::with_defaults(tran)
					.execute(Server.serve())
					.for_each(|request| async move {
						tokio::spawn(request);
					})
					.await;
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
