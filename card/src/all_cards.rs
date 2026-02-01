use crate::{Rank, Suit};

pub struct AllCards {
	suit: &'static [Suit],
	rank: &'static [Rank],
	suit_idx: usize,
	rank_idx: usize,
}
