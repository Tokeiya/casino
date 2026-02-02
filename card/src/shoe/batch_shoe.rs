use super::prelude::{Error as ShoeError, Result as ShoeResult};
use crate::prelude::*;
use crate::shoe::shoe_trait::Shoe;
use common_errors::invalid_argument::Information;
use rand::Rng;
use rand::prelude::*;

pub struct BachShoe<'a, R, F, I> {
	scr: F,
	rnd: R,
	cut: usize,
	limit: usize,
	store: Vec<Card>,
	phantom: std::marker::PhantomData<&'a I>,
}

impl<'a, R, F, I> BachShoe<'a, R, F, I>
where
	R: RngCore,
	I: IntoIterator<Item = &'a Card>,
	F: FnMut() -> I,
{
	fn new(scr: F, rnd: R, cut: usize, limit: usize) -> ShoeResult<Self> {
		todo!()
	}
}

impl<'a, R, F, I> Shoe for BachShoe<'a, R, F, I>
where
	R: RngCore,
	I: IntoIterator<Item = &'a Card>,
	F: FnMut() -> I,
{
	fn try_deal(&mut self) -> ShoeResult<Card> {
		todo!()
	}

	fn round_end(&mut self) {
		todo!()
	}

	fn remaining(&self) -> usize {
		todo!()
	}

	fn reach_cut(&self) -> bool {
		todo!()
	}

	fn reach_end(&self) -> bool {
		todo!()
	}

	fn cut_remaining(&self) -> usize {
		todo!()
	}

	fn end_remaining(&self) -> usize {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::prelude::*;
	use rand::RngCore;
	use rand::seq::SliceRandom;
	use rand_chacha::ChaCha8Rng;

	#[test]
	fn new() {
		let mut rng = ChaCha8Rng::seed_from_u64(42);
		let mut expected = (0..6).flat_map(|_| CARDS.iter()).collect::<Vec<_>>();

		expected.shuffle(&mut rng);

		let rng = ChaCha8Rng::seed_from_u64(42);
		let fixture = BachShoe::new(|| (0..6).flat_map(|_| CARDS.iter()), rng, 60, 52).unwrap();
		assert_eq!(fixture.limit, 52);
		assert_eq!(fixture.cut, 60);

		assert_eq!(fixture.store.len(), expected.len());

		for (a, e) in fixture.store.iter().zip(expected.iter()) {
			assert_eq!(a, *e);
		}
	}

	#[test]
	fn new_cut_over_size() {
		let rng = ChaCha8Rng::seed_from_u64(42);
		let fixture = BachShoe::new(|| (0..6).flat_map(|_| CARDS.iter()), rng, 318, 319);
	}
}
