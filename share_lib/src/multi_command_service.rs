#[tarpc::service]
pub trait Service {
	async fn add(value: u64) -> u64;
	async fn remove(value: u64) -> bool;
	async fn dump() -> Vec<u64>;
}
