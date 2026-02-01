use crate::card::Card;
use crate::shoe::error::Result;

pub trait Shoe {
	fn try_deal(&mut self) -> Result<Card>;
	fn round_end(&mut self);
	fn remaining(&self) -> usize;
	fn reach_cut(&self) -> bool;
	fn reach_end(&self) -> bool;
	fn cut_remaining(&self) -> usize;
	fn end_remaining(&self) -> usize;
}
