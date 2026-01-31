use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Rank {
	Ace,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
}

impl Display for Rank {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Rank::Ace => write!(f, "A"),
			Rank::Two => write!(f, "2"),
			Rank::Three => write!(f, "3"),
			Rank::Four => write!(f, "4"),
			Rank::Five => write!(f, "5"),
			Rank::Six => write!(f, "6"),
			Rank::Seven => write!(f, "7"),
			Rank::Eight => write!(f, "8"),
			Rank::Nine => write!(f, "9"),
			Rank::Ten => write!(f, "X"),
			Rank::Jack => write!(f, "J"),
			Rank::Queen => write!(f, "Q"),
			Rank::King => write!(f, "K"),
		}
	}
}

#[cfg(test)]
pub mod sample {
	use super::*;

	pub const RANKS: [Rank; 13] = [
		Rank::Ace,
		Rank::Two,
		Rank::Three,
		Rank::Four,
		Rank::Five,
		Rank::Six,
		Rank::Seven,
		Rank::Eight,
		Rank::Nine,
		Rank::Ten,
		Rank::Jack,
		Rank::Queen,
		Rank::King,
	];
}

#[cfg(test)]
mod tests {
	use super::sample::RANKS;
	use super::*;
	use std::sync::LazyLock;

	static EXPECTED: LazyLock<[&str; 13]> = LazyLock::new(|| {
		[
			"A", "2", "3", "4", "5", "6", "7", "8", "9", "X", "J", "Q", "K",
		]
	});

	//noinspection DuplicatedCode
	#[test]
	fn display_test() {
		for (rank, expected) in RANKS.iter().zip(EXPECTED.iter()) {
			assert_eq!(rank.to_string(), *expected);
		}
	}
}
