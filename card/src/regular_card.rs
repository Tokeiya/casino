use crate::rank::Rank;
use crate::suit::Suit;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct RegularCard {
	pub suit: Suit,
	pub rank: Rank,
}

impl RegularCard {
	pub fn new(suit: Suit, rank: Rank) -> Self {
		Self { suit, rank }
	}
}

impl Display for RegularCard {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.suit.fmt(f)?;
		write!(f, "_")?;
		self.rank.fmt(f)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::rank::RANKS;
	use crate::suit::SUITS;

	#[test]
	fn test_new_card() {
		let card = RegularCard::new(Suit::Spades, Rank::Ace);
		assert_eq!(card.suit, Suit::Spades);
		assert_eq!(card.rank, Rank::Ace);
	}

	#[test]
	fn test_display() {
		let iter = SUITS
			.iter()
			.flat_map(|s| RANKS.iter().map(move |r| (s, r)))
			.map(|(s, r)| (format!("{s}_{r}"), RegularCard::new(*s, *r)));

		for (e, a) in iter {
			assert_eq!(a.to_string(), e);
		}
	}
}
