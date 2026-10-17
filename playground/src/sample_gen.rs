use super::datum::Datum;
use rand::Rng;

pub struct SampleGen<R: Rng + Clone> {
	rng: R,
	candidates: Vec<Datum>,
}
