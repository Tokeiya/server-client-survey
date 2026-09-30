mod command;

use share_lib::RemoteInterface;
use tarpc::context::Context;

#[derive(Debug, Clone)]
pub struct Server;

impl RemoteInterface for Server {
	async fn say_hello(self, _context: Context, name: String) -> String {
		println!("Hello {name}");
		format!("Hello, {}!", name)
	}
}
