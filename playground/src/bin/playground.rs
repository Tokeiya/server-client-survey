use rand_core::SeedableRng;

fn main() {}

fn foo<S: Sized + Default + AsRef<[u8]> + AsMut<[u8]>, T: SeedableRng<Seed = S>>(seed: S) {
	let mut rnd = T::from_seed(seed);
}
