use rand::seq::SliceRandom;
use rand::{Rng, TryRng};
use std::convert::Infallible;
struct Dummy;

impl TryRng<Error = Infallible> for Dummy {
	type Error = ();

	fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
		todo!()
	}

	fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
		todo!()
	}

	fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
		todo!()
	}
}

impl Rng for Dummy {
	fn next_u32(&mut self) -> u32 {
		todo!()
	}

	fn next_u64(&mut self) -> u64 {
		todo!()
	}

	fn fill_bytes(&mut self, dst: &mut [u8]) {
		todo!()
	}
}

fn main() {
	let mut dummy = Dummy;

	let mut vec = vec![1, 2, 3];
	vec.shuffle(&mut dummy);
}
