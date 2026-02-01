mod all_cards;
mod card;
mod rank;
mod regular_card;
mod shoe;
mod suit;

pub use card::Card;
pub use rank::{RANKS, Rank};
pub use regular_card::RegularCard;
pub use suit::{SUITS, Suit};

pub fn all_pairs() -> impl Iterator<Item = (Rank, Suit)> {
	SUITS
		.iter()
		.flat_map(|&suit| RANKS.iter().map(move |&rank| (rank, suit)))
}

// pub fn all_cards(contain_joker: bool) -> impl Iterator<Item = Card> {
// }
