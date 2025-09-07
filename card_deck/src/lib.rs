#[derive(PartialEq, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(PartialEq, Debug)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        use Suit::*;
        match rand::random::<u8>() % 4 {
            0 => Heart,
            1 => Diamond,
            2 => Spade,
            _ => Club,
        }
    }

    pub fn translate(value: u8) -> Suit {
        use Suit::*;
        match value {
            1 => Heart,
            2 => Diamond,
            3 => Spade,
            _ => Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        use Rank::*;
        match rand::random::<u8>() % 5 {
            0 => Ace,
            1 => King,
            2 => Queen,
            3 => Jack,
            _ => Number(rand::random::<u8>() % 10 + 1),
        }
    }

    pub fn translate(value: u8) -> Rank {
        use Rank::*;
        match value {
            1 => Ace,
            11 => Jack,
            12 => Queen,
            13 => King,
            _ => Number(value),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
