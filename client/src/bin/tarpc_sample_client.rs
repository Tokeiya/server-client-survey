use share_lib::RemoteInterfaceClient;
use tarpc::{client, context, serde_transport::unix, tokio_serde::formats::Bincode};

use tokio::runtime::Builder;

async fn proc_main() {
	let transport = unix::connect("/tmp/sock", Bincode::default).await.unwrap();
	let client = RemoteInterfaceClient::new(client::Config::default(), transport).spawn();

	let response = client
		.say_hello(context::current(), "時計屋".to_owned())
		.await
		.unwrap();
	println!("received response: {}", response);

	for i in 0..usize::MAX {
		let response = client
			.say_hello(context::current(), format!("時計屋{}", i))
			.await
			.unwrap();
		println!("received response: {}", response);
		tokio::time::sleep(std::time::Duration::from_millis(100)).await;
	}
}
fn main() {
	println!("build runtime");
	let rt = Builder::new_multi_thread().enable_all().build().unwrap();
	println!("send message");
	rt.block_on(proc_main());
}
