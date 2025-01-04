use std::{
    fmt::Debug,
    sync::atomic::{AtomicU32, Ordering},
};

use colored::Colorize;
use rand::seq::SliceRandom;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum CardSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Debug for CardSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            CardSuit::Spades => "♠️".black(),
            CardSuit::Hearts => "♥️".red(),
            CardSuit::Diamonds => "♦️".yellow(),
            CardSuit::Clubs => "♣️".green(),
        };
        write!(f, "{}", txt)
    }
}

pub const CARD_SUITS: [CardSuit; 4] = [
    CardSuit::Spades,
    CardSuit::Hearts,
    CardSuit::Diamonds,
    CardSuit::Clubs,
];

impl CardSuit {
    pub fn get_rand() -> Self {
        CARD_SUITS.choose(&mut rand::thread_rng()).unwrap().clone()
    }
}

#[derive(Clone, Debug)]
pub enum CardEnhancement {
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

#[derive(Clone, Debug)]
pub enum CardEdition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CardSeal {
    Gold,
    Red,
    Blue,
    Purple,
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardRank {
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

impl Debug for CardRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            CardRank::Ace => "A",
            CardRank::Two => "2",
            CardRank::Three => "3",
            CardRank::Four => "4",
            CardRank::Five => "5",
            CardRank::Six => "6",
            CardRank::Seven => "7",
            CardRank::Eight => "8",
            CardRank::Nine => "9",
            CardRank::Ten => "10",
            CardRank::Jack => "J",
            CardRank::Queen => "Q",
            CardRank::King => "K",
        };
        write!(f, "{}", txt)
    }
}

pub const CARD_RANKS: [CardRank; 13] = [
    CardRank::Two,
    CardRank::Three,
    CardRank::Four,
    CardRank::Five,
    CardRank::Six,
    CardRank::Seven,
    CardRank::Eight,
    CardRank::Nine,
    CardRank::Ten,
    CardRank::Jack,
    CardRank::Queen,
    CardRank::King,
    CardRank::Ace,
];

impl CardRank {
    pub fn get_rand() -> Self {
        CARD_RANKS.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn get_straight_next(&self) -> Self {
        match self {
            CardRank::Two => CardRank::Three,
            CardRank::Three => CardRank::Four,
            CardRank::Four => CardRank::Five,
            CardRank::Five => CardRank::Six,
            CardRank::Six => CardRank::Seven,
            CardRank::Seven => CardRank::Eight,
            CardRank::Eight => CardRank::Nine,
            CardRank::Nine => CardRank::Ten,
            CardRank::Ten => CardRank::Jack,
            CardRank::Jack => CardRank::Queen,
            CardRank::Queen => CardRank::King,
            CardRank::King => CardRank::Ace,
            CardRank::Ace => CardRank::Two,
        }
    }

    pub fn get_straight_prev(&self) -> Self {
        match self {
            CardRank::Two => CardRank::Ace,
            CardRank::Three => CardRank::Two,
            CardRank::Four => CardRank::Three,
            CardRank::Five => CardRank::Four,
            CardRank::Six => CardRank::Five,
            CardRank::Seven => CardRank::Six,
            CardRank::Eight => CardRank::Seven,
            CardRank::Nine => CardRank::Eight,
            CardRank::Ten => CardRank::Nine,
            CardRank::Jack => CardRank::Ten,
            CardRank::Queen => CardRank::Jack,
            CardRank::King => CardRank::Queen,
            CardRank::Ace => CardRank::King,
        }
    }

    pub fn get_base_chips(&self) -> u32 {
        match self {
            CardRank::Two => 2,
            CardRank::Three => 3,
            CardRank::Four => 4,
            CardRank::Five => 5,
            CardRank::Six => 6,
            CardRank::Seven => 7,
            CardRank::Eight => 8,
            CardRank::Nine => 9,
            CardRank::Ten => 10,

            CardRank::Jack => 10,
            CardRank::Queen => 10,
            CardRank::King => 10,

            CardRank::Ace => 11,
        }
    }
}

pub const FACE_CARDS: [CardRank; 3] = [CardRank::Jack, CardRank::Queen, CardRank::King];

type CardId = u32;

#[derive(Clone)]
pub struct Card {
    pub id: CardId,

    pub rank: CardRank,
    pub suit: CardSuit,
    pub enhancement: Option<CardEnhancement>,
    pub edition: Option<CardEdition>,
    pub seal: Option<CardSeal>,

    pub extra_chips: u32,
}

impl Card {
    fn get_next_id() -> CardId {
        static NEXT_ID: AtomicU32 = AtomicU32::new(0);
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    }

    pub fn new(rank: CardRank, suit: CardSuit) -> Self {
        Self {
            id: Self::get_next_id(),
            rank,
            suit,
            enhancement: None,
            edition: None,
            seal: None,
            extra_chips: 0,
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?} {:?}]", self.rank, self.suit)
    }
}
