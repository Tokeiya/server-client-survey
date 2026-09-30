use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
	Add(u64),
	Remove(u64),
	Dump,
}
