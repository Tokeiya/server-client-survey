#[tarpc::service]
pub trait RemoteInterface {
	async fn say_hello(name: String) -> String;
}
