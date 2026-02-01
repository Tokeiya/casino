use super::prelude::{Error as ShoeError, Result as ShoeResult};
use crate::prelude::*;
use crate::shoe::shoe_trait::Shoe;
use common_errors::invalid_argument::Information;
use rand::Rng;
use rand::prelude::*;

pub struct BachShoe<R> {
	scr: Vec<Card>,
	rnd: R,
	cut: usize,
	limit: usize,
	store: Vec<Card>,
}

impl<R> BachShoe<R> {
	fn new(scr: Vec<Card>, rnd: R, cut: usize, limit: usize) -> ShoeResult<Self> {
		todo!()
	}
}

impl<R: Rng> Shoe for BachShoe<R> {
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
mod tests {}
