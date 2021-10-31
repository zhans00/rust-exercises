use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
	Heart,
	Diamond,
	Spade,
	Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
	Ace,
	King,
	Queen,
	Jack,
	Number,
}

impl Suit {
	pub fn random() -> Suit {
		let r =  rand::thread_rng().gen_range(0, 4);
		if r == 0 {
			return Suit::Heart;
		} else if r == 1 {
			return Suit::Diamond;
		} else if r == 2 {
			return Suit::Spade;
		} else {
			return Suit::Club;
		}
	}

	pub fn translate(value: u8) -> Suit {
		if value == 1 {
			return Suit::Heart;
		} else if value == 2 {
			return Suit::Diamond;
		} else if value == 3 {
			return Suit::Spade;
		} else {
			return Suit::Club;
		}
	}
}

impl Rank {
	pub fn random() -> Rank {
		let r = rand::thread_rng().gen_range(0, 5);
		if r == 0 {
			return Rank::Number;
		} else if r == 1 {
			return Rank::Queen;
		} else if r == 2 {
			return Rank::King;
		} else if r == 3 {
			return Rank::Jack;
		} else {
			return Rank::Ace;
		}
	}

	pub fn translate(value: u8) -> Rank {
		if value >= 2 && value <= 10 {
			return Rank::Number;
		}
		if value == 1 { 
			return Rank::Ace;
		} else if value == 11 {
			return Rank::Jack;
		} else if value == 12 {
			return Rank::Queen;
		} else {
			return Rank::King;
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card:Card) -> bool {
	if card.rank == Rank::Ace && card.suit == Suit::Spade {
		return true;
	}

	return false;
}