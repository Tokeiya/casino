use crate::rank::Rank;
use crate::regular_card::RegularCard;
use crate::suit::Suit;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Card {
	Regular(RegularCard),
	Joker,
}

impl Card {
	pub fn new(suit: Suit, rank: Rank) -> Self {
		Card::Regular(RegularCard::new(suit, rank))
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Card::Regular(r) => write!(f, "{r}"),
			Card::Joker => write!(f, "Jkr"),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::rank::RANKS;
	use crate::suit::SUITS;

	#[test]
	fn new() {
		let iter = RANKS
			.iter()
			.flat_map(|r| SUITS.iter().map(move |s| (r, s)))
			.map(|(r, s)| (Card::new(*s, *r), (*s, *r)));

		for (actual, (suit, rank)) in iter {
			match actual {
				Card::Regular(c) => {
					assert_eq!(c.suit, suit);
					assert_eq!(c.rank, rank);
				}
				Card::Joker => unreachable!(),
			}
		}
	}

	#[test]
	fn display() {
		assert_eq!(Card::Joker.to_string(), "Jkr");

		let iter = RANKS
			.iter()
			.flat_map(|r| SUITS.iter().map(move |s| (r, s)))
			.map(|(r, s)| (Card::new(*s, *r), RegularCard::new(*s, *r).to_string()));

		for (actual, expected) in iter {
			assert_eq!(actual.to_string(), expected);
		}
	}
}
