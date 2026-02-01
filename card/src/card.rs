use super::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Card {
	Regular(RegularCard),
	Joker,
}

impl Card {
	pub const fn from_value(suit: Suit, rank: Rank) -> Self {
		Card::Regular(RegularCard::new(suit, rank))
	}
	pub fn from_ref(suit: &Suit, rank: &Rank) -> Self {
		Card::from_value(*suit, *rank)
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

pub const CARDS: [Card; 53] = [
	Card::Joker,
	Card::from_value(Suit::Spades, Rank::Ace),
	Card::from_value(Suit::Spades, Rank::Two),
	Card::from_value(Suit::Spades, Rank::Three),
	Card::from_value(Suit::Spades, Rank::Four),
	Card::from_value(Suit::Spades, Rank::Five),
	Card::from_value(Suit::Spades, Rank::Six),
	Card::from_value(Suit::Spades, Rank::Seven),
	Card::from_value(Suit::Spades, Rank::Eight),
	Card::from_value(Suit::Spades, Rank::Nine),
	Card::from_value(Suit::Spades, Rank::Ten),
	Card::from_value(Suit::Spades, Rank::Jack),
	Card::from_value(Suit::Spades, Rank::Queen),
	Card::from_value(Suit::Spades, Rank::King),
	Card::from_value(Suit::Hearts, Rank::Ace),
	Card::from_value(Suit::Hearts, Rank::Two),
	Card::from_value(Suit::Hearts, Rank::Three),
	Card::from_value(Suit::Hearts, Rank::Four),
	Card::from_value(Suit::Hearts, Rank::Five),
	Card::from_value(Suit::Hearts, Rank::Six),
	Card::from_value(Suit::Hearts, Rank::Seven),
	Card::from_value(Suit::Hearts, Rank::Eight),
	Card::from_value(Suit::Hearts, Rank::Nine),
	Card::from_value(Suit::Hearts, Rank::Ten),
	Card::from_value(Suit::Hearts, Rank::Jack),
	Card::from_value(Suit::Hearts, Rank::Queen),
	Card::from_value(Suit::Hearts, Rank::King),
	Card::from_value(Suit::Diamonds, Rank::Ace),
	Card::from_value(Suit::Diamonds, Rank::Two),
	Card::from_value(Suit::Diamonds, Rank::Three),
	Card::from_value(Suit::Diamonds, Rank::Four),
	Card::from_value(Suit::Diamonds, Rank::Five),
	Card::from_value(Suit::Diamonds, Rank::Six),
	Card::from_value(Suit::Diamonds, Rank::Seven),
	Card::from_value(Suit::Diamonds, Rank::Eight),
	Card::from_value(Suit::Diamonds, Rank::Nine),
	Card::from_value(Suit::Diamonds, Rank::Ten),
	Card::from_value(Suit::Diamonds, Rank::Jack),
	Card::from_value(Suit::Diamonds, Rank::Queen),
	Card::from_value(Suit::Diamonds, Rank::King),
	Card::from_value(Suit::Clubs, Rank::Ace),
	Card::from_value(Suit::Clubs, Rank::Two),
	Card::from_value(Suit::Clubs, Rank::Three),
	Card::from_value(Suit::Clubs, Rank::Four),
	Card::from_value(Suit::Clubs, Rank::Five),
	Card::from_value(Suit::Clubs, Rank::Six),
	Card::from_value(Suit::Clubs, Rank::Seven),
	Card::from_value(Suit::Clubs, Rank::Eight),
	Card::from_value(Suit::Clubs, Rank::Nine),
	Card::from_value(Suit::Clubs, Rank::Ten),
	Card::from_value(Suit::Clubs, Rank::Jack),
	Card::from_value(Suit::Clubs, Rank::Queen),
	Card::from_value(Suit::Clubs, Rank::King),
];

#[cfg(test)]
mod test {
	use super::*;
	use crate::rank::RANKS;
	use crate::suit::SUITS;

	//noinspection DuplicatedCode
	#[test]
	fn from_value() {
		let iter = RANKS
			.iter()
			.flat_map(|r| SUITS.iter().map(move |s| (r, s)))
			.map(|(r, s)| (Card::from_value(*s, *r), (*s, *r)));

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

	//noinspection DuplicatedCode
	#[test]
	fn from_ref() {
		let iter = RANKS
			.iter()
			.flat_map(|r| SUITS.iter().map(move |s| (r, s)))
			.map(move |(r, s)| (Card::from_ref(s, r), (*s, *r)));

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
			.map(|(r, s)| {
				(
					Card::from_value(*s, *r),
					RegularCard::new(*s, *r).to_string(),
				)
			});

		for (actual, expected) in iter {
			assert_eq!(actual.to_string(), expected);
		}
	}

	#[test]
	fn cards_verify() {
		assert_eq!(CARDS.len(), 53);
		assert_eq!(CARDS[0], Card::Joker);

		let expected = SUITS.iter().flat_map(|s| RANKS.iter().map(move |r| (s, r)));

		let mut cnt = 0usize;

		for (a, (s, r)) in CARDS[1..].iter().zip(expected) {
			cnt += 1;
			let a = match a {
				Card::Regular(c) => c,
				Card::Joker => unreachable!(),
			};

			assert_eq!(&a.suit, s);
			assert_eq!(&a.rank, r);
		}

		assert_eq!(cnt, 52);
	}
}
