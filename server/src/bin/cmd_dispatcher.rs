use share_lib::multi_command_service::*;
use std::process::Command;
use tarpc::tokio_serde::formats::Bincode;
use tarpc::{ClientMessage, Response};
use tokio::sync::mpsc::UnboundedSender;

type RequestMessage = ClientMessage<ServiceRequest>;
type ResponseMessage = Response<ServiceResponse>;
type Codec = Bincode<RequestMessage, ResponseMessage>;

type Incoming =
	tarpc::serde_transport::unix::Incoming<RequestMessage, ResponseMessage, Codec, fn() -> Codec>;

pub struct ServiceImpl {
	tx: UnboundedSender<Command>,
	listener: Incoming,
}

impl ServiceImpl {
	pub fn new(_tx: UnboundedSender<Command>, _path: String) -> Self {
		todo!()
		//Self { tx }
	}
}

fn main() {}
