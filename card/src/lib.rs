mod card;
mod rank;
mod regular_card;
mod shoe;
mod suit;

pub mod prelude {
	pub use super::card::{CARDS, Card};
	pub use super::rank::{RANKS, Rank};
	pub use super::regular_card::RegularCard;
	pub use super::suit::{SUITS, Suit};
}

use prelude::*;

pub fn suit_rank_cartesian() -> impl Iterator<Item = (&'static Suit, &'static Rank)> {
	SUITS
		.iter()
		.flat_map(|suit| RANKS.iter().map(move |rank| (suit, rank)))
}

pub fn with_joker() -> std::slice::Iter<'static, Card> {
	CARDS.iter()
}

pub fn without_joker() -> std::slice::Iter<'static, Card> {
	CARDS[1..].iter()
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashMap;
	#[test]
	fn suit_rank_cartesian_test() {
		let mut suits: HashMap<&'static Suit, usize> = SUITS.iter().map(|s| (s, 0usize)).collect();
		let mut ranks: HashMap<&'static Rank, usize> = RANKS.iter().map(|r| (r, 0usize)).collect();

		for (s, r) in suit_rank_cartesian() {
			*suits.get_mut(s).unwrap() += 1;
			*ranks.get_mut(r).unwrap() += 1;
		}

		assert!(suits.iter().all(|(_, c)| c == &13));
		assert!(ranks.iter().all(|(_, c)| c == &4));
	}

	#[test]
	fn with_joker_test() {
		assert_eq!(with_joker().len(), 53);

		let mut suits: HashMap<&'static Suit, usize> = SUITS.iter().map(|s| (s, 0usize)).collect();
		let mut ranks: HashMap<&'static Rank, usize> = RANKS.iter().map(|r| (r, 0usize)).collect();

		let mut flg = false;

		for (a, e) in with_joker().zip(CARDS.iter()) {
			assert_eq!(a, e);

			let a = match a {
				Card::Regular(c) => c,
				Card::Joker => {
					assert!(!flg);
					flg = true;
					continue;
				}
			};

			*suits.get_mut(&a.suit).unwrap() += 1;
			*ranks.get_mut(&a.rank).unwrap() += 1;
		}

		assert!(flg);
		assert!(suits.iter().all(|(_, c)| c == &13));
		assert!(ranks.iter().all(|(_, c)| c == &4));
	}

	#[test]
	fn without_joker_test() {
		assert_eq!(with_joker().len(), 53);

		let mut suits: HashMap<&'static Suit, usize> = SUITS.iter().map(|s| (s, 0usize)).collect();
		let mut ranks: HashMap<&'static Rank, usize> = RANKS.iter().map(|r| (r, 0usize)).collect();

		for (a, e) in without_joker().zip(CARDS[1..].iter()) {
			assert_eq!(a, e);

			let a = match a {
				Card::Regular(c) => c,
				Card::Joker => unreachable!(),
			};

			*suits.get_mut(&a.suit).unwrap() += 1;
			*ranks.get_mut(&a.rank).unwrap() += 1;
		}

		assert!(suits.iter().all(|(_, c)| c == &13));
		assert!(ranks.iter().all(|(_, c)| c == &4));
	}
}
