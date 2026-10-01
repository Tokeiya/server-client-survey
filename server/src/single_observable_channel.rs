use std::cell::RefCell;
use std::cell::{Cell, Ref};
use std::collections::VecDeque;
use std::rc::Rc;
use tokio::sync::Notify;

struct ObservableChannel<T> {
	storage: RefCell<VecDeque<T>>,
	notify: Notify,
	sender_count: Cell<usize>,
}

impl<T> ObservableChannel<T> {
	fn new() -> Self {
		Self {
			storage: RefCell::new(VecDeque::new()),
			notify: Notify::new(),
			sender_count: Cell::new(0),
		}
	}

	fn enqueue(&self, value: T) {
		self.storage.borrow_mut().push_back(value);
		self.notify.notify_one();
	}

	async fn dequeue(&self) -> Option<T> {
		loop {
			let value = {
				let mut storage = self.storage.borrow_mut();
				storage.pop_front()
			};

			if let Some(value) = value {
				return Some(value);
			} else if self.sender_count.get() == 0 {
				return None;
			}

			self.notify.notified().await;
		}
	}

	fn len(&self) -> usize {
		self.storage.borrow().len()
	}

	fn borrow_storage(&self) -> Ref<'_, VecDeque<T>> {
		self.storage.borrow()
	}

	fn increment_count(&self) {
		self.sender_count.set(self.sender_count.get() + 1);
	}

	fn decrement_count(&self) -> bool {
		self.sender_count.set(self.sender_count.get() - 1);
		self.sender_count.get() == 0
	}

	fn sender_count(&self) -> usize {
		self.sender_count.get()
	}
}

struct Receiver<T> {
	observable_channel: Rc<ObservableChannel<T>>,
}

impl<T> Receiver<T> {
	fn new(observable_channel: Rc<ObservableChannel<T>>) -> Self {
		Self { observable_channel }
	}

	async fn dequeue(&self) -> Option<T> {
		self.observable_channel.dequeue().await
	}

	fn len(&self) -> usize {
		self.observable_channel.len()
	}

	fn borrow_storage(&self) -> Ref<'_, VecDeque<T>> {
		self.observable_channel.borrow_storage()
	}
}

struct Sender<T> {
	observable_channel: Rc<ObservableChannel<T>>,
}

impl<T> Sender<T> {
	fn new(observable_channel: Rc<ObservableChannel<T>>) -> Self {
		observable_channel.increment_count();
		Self { observable_channel }
	}

	fn enqueue(&self, value: T) {
		self.observable_channel.enqueue(value);
	}
}

impl<T> Drop for Sender<T> {
	fn drop(&mut self) {
		let cnt = self.observable_channel.decrement_count();

		if cnt {
			self.observable_channel.notify.notify_waiters();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::future::{Future, poll_fn};
	use std::task::Poll;
	use std::time::Duration;
	use tokio::sync::oneshot;
	use tokio::task::{JoinHandle, LocalSet};
	use tokio::time::timeout;

	// 生成方法を変更するときは、まずここを変更する。
	fn setup<T>() -> (Sender<T>, Receiver<T>) {
		let channel = Rc::new(ObservableChannel::new());

		(Sender::new(Rc::clone(&channel)), Receiver::new(channel))
	}

	// ハングをテスト失敗として扱う。
	async fn run_local(future: impl Future<Output = ()>) {
		timeout(Duration::from_secs(3), LocalSet::new().run_until(future))
			.await
			.expect("チャネルの処理が完了しませんでした");
	}

	// dequeue() が実際に Pending になったことを確認してから戻る。
	// 戻った後で enqueue / Drop することで、待機解除を検証できる。
	async fn start_waiting(receiver: Rc<Receiver<i32>>) -> JoinHandle<Option<i32>> {
		let (started_tx, started_rx) = oneshot::channel();

		let task = tokio::task::spawn_local(async move {
			let receive = receiver.dequeue();
			tokio::pin!(receive);

			poll_fn(|cx| {
				assert!(
					receive.as_mut().poll(cx).is_pending(),
					"送信者が存在する空のチャネルは待機するべきです"
				);

				Poll::Ready(())
			})
			.await;

			started_tx.send(()).unwrap();

			receive.await
		});

		started_rx
			.await
			.expect("受信タスクが待機に入る前に終了しました");

		task
	}

	// 読み取りによる観察では、キューを消費しない。
	#[tokio::test(flavor = "current_thread")]
	async fn observing_does_not_consume_values() {
		run_local(async {
			let (sender, receiver) = setup::<i32>();

			assert_eq!(receiver.len(), 0);

			sender.enqueue(10);
			sender.enqueue(20);

			assert_eq!(receiver.len(), 2);

			{
				let storage = receiver.borrow_storage();
				let values: Vec<_> = storage.iter().copied().collect();

				assert_eq!(values, vec![10, 20]);
			}

			assert_eq!(receiver.len(), 2);
			assert_eq!(receiver.dequeue().await, Some(10));
			assert_eq!(receiver.len(), 1);
			assert_eq!(receiver.dequeue().await, Some(20));
			assert_eq!(receiver.len(), 0);
		})
		.await;
	}

	// 先に複数回送信しても、通知の回数に依存せず全件取り出せる。
	// Sender を残すことで、終了通知に助けられずに取り出せるか確認する。
	#[tokio::test(flavor = "current_thread")]
	async fn queued_values_are_received_in_fifo_order() {
		run_local(async {
			let (sender, receiver) = setup::<i32>();

			for value in 0..5 {
				sender.enqueue(value);
			}

			for expected in 0..5 {
				assert_eq!(receiver.dequeue().await, Some(expected));
			}

			assert_eq!(receiver.len(), 0);
			drop(sender);
		})
		.await;
	}

	// 待機中の受信処理を、送信によって起こせる。
	#[tokio::test(flavor = "current_thread")]
	async fn enqueue_wakes_waiting_receiver() {
		run_local(async {
			let (sender, receiver) = setup::<i32>();
			let receiver = Rc::new(receiver);

			let task = start_waiting(receiver).await;

			sender.enqueue(42);

			assert_eq!(task.await.unwrap(), Some(42));
		})
		.await;
	}

	// 送信者が先に全滅していても、残っている値は失われない。
	#[tokio::test(flavor = "current_thread")]
	async fn buffered_values_are_drained_before_end() {
		run_local(async {
			let (sender, receiver) = setup::<i32>();

			sender.enqueue(10);
			sender.enqueue(20);
			drop(sender);

			assert_eq!(receiver.dequeue().await, Some(10));
			assert_eq!(receiver.dequeue().await, Some(20));
			assert_eq!(receiver.dequeue().await, None);

			// 送信者を再生成しない限り、終了後も None。
			assert_eq!(receiver.dequeue().await, None);
		})
		.await;
	}

	// 待機を始める前に送信者が消えていても、待ち続けない。
	#[tokio::test(flavor = "current_thread")]
	async fn empty_channel_without_senders_returns_none() {
		run_local(async {
			let (sender, receiver) = setup::<i32>();

			drop(sender);

			assert_eq!(receiver.dequeue().await, None);
		})
		.await;
	}

	// 送信者が1つでも残っていれば、終了してはいけない。
	#[tokio::test(flavor = "current_thread")]
	async fn dropping_one_sender_does_not_close_channel() {
		run_local(async {
			let channel = Rc::new(ObservableChannel::<i32>::new());
			let sender1 = Sender::new(Rc::clone(&channel));
			let sender2 = Sender::new(Rc::clone(&channel));
			let receiver = Rc::new(Receiver::new(channel));

			drop(sender1);

			// カウント管理が誤って0になっていたら、ここで失敗する。
			let task = start_waiting(Rc::clone(&receiver)).await;

			sender2.enqueue(42);
			assert_eq!(task.await.unwrap(), Some(42));

			drop(sender2);
			assert_eq!(receiver.dequeue().await, None);
		})
		.await;
	}

	// 最後の送信者の Drop は、待機中の受信処理を全て起こす。
	#[tokio::test(flavor = "current_thread")]
	async fn dropping_last_sender_wakes_all_waiters() {
		run_local(async {
			let (sender, receiver) = setup::<i32>();
			let receiver = Rc::new(receiver);

			let first = start_waiting(Rc::clone(&receiver)).await;
			let second = start_waiting(Rc::clone(&receiver)).await;

			drop(sender);

			assert_eq!(first.await.unwrap(), None);
			assert_eq!(second.await.unwrap(), None);
		})
		.await;
	}
}
