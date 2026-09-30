pub mod multi_command_service;

#[tarpc::service]
pub trait RemoteInterface {
	async fn say_hello(name: String) -> String;
}
