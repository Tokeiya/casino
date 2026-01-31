use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Suit {
	Spades,
	Hearts,
	Diamonds,
	Clubs,
}

impl Display for Suit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Suit::Spades => write!(f, "♠"),
			Suit::Hearts => write!(f, "♥"),
			Suit::Diamonds => write!(f, "♦"),
			Suit::Clubs => write!(f, "♣"),
		}
	}
}

#[cfg(test)]
pub mod sample {
	use crate::suit::Suit;

	pub const SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
}

#[cfg(test)]
mod tests {
	use crate::suit::sample::SUITS;
	use std::sync::LazyLock;

	static EXPECTED: LazyLock<[&str; 4]> = LazyLock::new(|| ["♠", "♥", "♦", "♣"]);

	//noinspection DuplicatedCode
	#[test]
	fn display_test() {
		for (suit, expected) in SUITS.iter().zip(EXPECTED.iter()) {
			assert_eq!(suit.to_string(), *expected);
		}
	}
}
