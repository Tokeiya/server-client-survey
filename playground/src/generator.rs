use rand::{Rng, RngExt};

#[derive(Clone)]
pub struct Generator<R: Clone> {
	rng: R,
	step: usize,
}

impl<R: Clone + Rng> Generator<R> {
	pub fn new(rng: R, step: usize) -> Self {
		Self { rng, step }
	}

	pub fn next(&mut self) -> u8 {
		let ret = self.rng.random_range(0u8..=u8::MAX);
		self.step += 1;
		ret
	}
}
