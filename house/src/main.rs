use card::prelude::*;

fn main() {
	for (s, r) in card::suit_rank_cartesian() {
		let s = match s {
			Suit::Spades => "Suit::Spades",
			Suit::Hearts => "Suit::Hearts",
			Suit::Diamonds => "Suit::Diamonds",
			Suit::Clubs => "Suit::Clubs",
		};

		let r = match r {
			Rank::Ace => "Rank::Ace",
			Rank::Two => "Rank::Two",
			Rank::Three => "Rank::Three",
			Rank::Four => "Rank::Four",
			Rank::Five => "Rank::Five",
			Rank::Six => "Rank::Six",
			Rank::Seven => "Rank::Seven",
			Rank::Eight => "Rank::Eight",
			Rank::Nine => "Rank::Nine",
			Rank::Ten => "Rank::Ten",
			Rank::Jack => "Rank::Jack",
			Rank::Queen => "Rank::Queen",
			Rank::King => "Rank::King",
		};

		println!("Card::from_value({s}, {r}),");
	}
}
