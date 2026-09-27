use futures::StreamExt;
use share_lib::{
	RemoteInterface, RemoteInterfaceClient, RemoteInterfaceRequest, RemoteInterfaceResponse,
};
use tarpc::{
	client, context,
	serde_transport::unix,
	server::{BaseChannel, Channel},
	tokio_serde::formats::Bincode,
};

use tokio::runtime::Builder;

async fn proc_main() {
	let transport = unix::connect("/tmp/sock", Bincode::default).await.unwrap();
	let client = RemoteInterfaceClient::new(client::Config::default(), transport).spawn();

	let response = client
		.say_hello(context::current(), "時計屋".to_owned())
		.await
		.unwrap();
	println!("received response: {}", response);

	let response = client
		.say_hello(context::current(), "時計屋42".to_owned())
		.await
		.unwrap();

	println!("received response: {}", response);
}
fn main() {
	println!("build runtime");
	let rt = Builder::new_multi_thread().enable_all().build().unwrap();
	println!("send message");
	rt.block_on(proc_main());
}
