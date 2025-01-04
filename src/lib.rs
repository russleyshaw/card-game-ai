mod blind;
mod card;
mod consumable;
mod joker;
mod utils;
mod voucher;

use std::{
    clone,
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use blind::{Blind, BossBlind};
use card::{Card, CardEnhancement, CardRank, CardSuit, CARD_RANKS, CARD_SUITS, FACE_CARDS};
use consumable::{Consumable, TarotCard};
use joker::JokerCard;
use rand::seq::SliceRandom;
use voucher::Voucher;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, {{project-name}}!");
}

#[derive(Clone, Debug)]
pub enum GameStake {
    White,
    Red,
    Green,
    Black,
    Blue,
    Purple,
    Orange,
    Gold,
}

// https://balatrogame.fandom.com/wiki/Decks
#[derive(Clone, Debug)]
pub enum GameStartingDeck {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
    Magic,
    Nebula,
    Ghost,
    Abandoned,
    Checkered,
    Zodiac,
    Painted,
    Anaglyph,
    Plasma,
    Erratic,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,

    // Secret
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

impl HandType {
    pub fn from_cards(cards: Vec<&Card>) -> (Self, Vec<&Card>) {
        if cards.len() == 0 {
            panic!("No cards");
        }

        // Collect by suit
        let mut by_suits: HashMap<CardSuit, Vec<&Card>> = HashMap::new();
        cards.iter().for_each(|card| {
            by_suits
                .entry(card.suit.clone())
                .or_insert(vec![])
                .push(card);
        });

        let mut by_ranks: HashMap<CardRank, Vec<&Card>> = HashMap::new();
        cards.iter().for_each(|card| {
            by_ranks
                .entry(card.rank.clone())
                .or_insert(vec![])
                .push(card);
        });

        // sorted_by_value
        let mut sorted_by_value = cards.clone();
        sorted_by_value.sort_by(|a, b| b.rank.cmp(&a.rank));

        // Five of a kind
        let five_of_a_kind_cards = by_ranks.iter().find(|(_, cards)| cards.len() == 5);
        if let Some(five_of_a_kind_cards) = five_of_a_kind_cards {
            return (HandType::FiveOfAKind, five_of_a_kind_cards.1.clone());
        }

        // Four of a kind
        let four_of_a_kind_cards = by_ranks.iter().find(|(_, cards)| cards.len() == 4);
        if let Some(four_of_a_kind_cards) = four_of_a_kind_cards {
            return (HandType::FourOfAKind, four_of_a_kind_cards.1.clone());
        }

        // Flush
        let flush_cards = by_suits.iter().find(|(_, cards)| cards.len() == 5);
        if let Some(flush_cards) = flush_cards {
            return (HandType::Flush, flush_cards.1.clone());
        }

        // Straight
        let straight_cards_needed = 5;
        if sorted_by_value.len() >= straight_cards_needed {
            let mut straight_cards = vec![];

            for i in 0..sorted_by_value.len() {
                straight_cards = vec![sorted_by_value[i]];

                for j in i + 1..straight_cards_needed {
                    let prev_card = sorted_by_value[j - 1];
                    let curr_card = sorted_by_value[j];

                    if curr_card.rank == prev_card.rank.get_straight_prev() {
                        straight_cards.push(sorted_by_value[j]);
                    }
                }

                if straight_cards.len() == straight_cards_needed {
                    return (HandType::Straight, straight_cards);
                }
            }
        }

        // At least three of a kind; maybe full house
        let triple_cards = by_ranks.iter().find(|(_, cards)| cards.len() == 3);
        let first_pair_cards = by_ranks.iter().find(|(_, cards)| cards.len() == 2);
        if let Some(first_triple_cards) = triple_cards {
            // Full house
            if let Some(full_house_pair_cards) = first_pair_cards {
                let mut full_house_cards = first_triple_cards.1.clone();
                full_house_cards.extend(full_house_pair_cards.1.clone());
                return (HandType::FullHouse, full_house_cards);
            }

            // Three of a kind
            return (HandType::ThreeOfAKind, first_triple_cards.1.clone());
        }

        // At least one pair; maybe two pair
        if let Some(first_pair_cards) = first_pair_cards {
            let second_pair_cards = by_ranks
                .iter()
                .find(|(rank, cards)| cards.len() == 2 && *rank != first_pair_cards.0);

            // Two pair
            if let Some(second_pair_cards) = second_pair_cards {
                let mut two_pair_cards = first_pair_cards.1.clone();
                two_pair_cards.extend(second_pair_cards.1.clone());
                return (HandType::TwoPair, two_pair_cards);
            }

            // Pair
            return (HandType::Pair, first_pair_cards.1.clone());
        }

        // If nothing else, return high card
        (Self::HighCard, vec![sorted_by_value.first().unwrap()])
    }
}

#[derive(Clone, Debug)]
pub struct ChipsAndMult {
    pub chips: u32,
    pub mult: u32,
}

#[derive(Clone, Debug)]
pub struct HandTypeLevel {
    pub high_card: u8,
    pub pair: u8,
    pub two_pair: u8,
    pub three_of_a_kind: u8,
    pub straight: u8,
    pub flush: u8,
    pub full_house: u8,
    pub four_of_a_kind: u8,
    pub straight_flush: u8, // And royal flush

    // Secret
    pub five_of_a_kind: u8,
    pub flush_house: u8,
    pub flush_five: u8,
}

impl HandTypeLevel {
    pub fn new() -> Self {
        Self {
            high_card: 0,
            pair: 0,
            two_pair: 0,
            three_of_a_kind: 0,
            straight: 0,
            flush: 0,
            full_house: 0,
            four_of_a_kind: 0,
            straight_flush: 0,

            // Secret
            five_of_a_kind: 0,
            flush_house: 0,
            flush_five: 0,
        }
    }

    pub fn get_value(&self, hand_type: &HandType) -> ChipsAndMult {
        match hand_type {
            HandType::HighCard => ChipsAndMult {
                mult: 1 + (self.high_card * 1) as u32,
                chips: 5 + (self.high_card * 5) as u32,
            },
            HandType::Pair => ChipsAndMult {
                mult: 2 + (self.pair * 1) as u32,
                chips: 10 + (self.pair * 15) as u32,
            },
            HandType::TwoPair => ChipsAndMult {
                mult: 2 + (self.two_pair * 1) as u32,
                chips: 20 + (self.two_pair * 20) as u32,
            },
            HandType::ThreeOfAKind => ChipsAndMult {
                mult: 3 + (self.three_of_a_kind * 2) as u32,
                chips: 30 + (self.three_of_a_kind * 20) as u32,
            },
            HandType::Straight => ChipsAndMult {
                mult: 4 + (self.straight * 3) as u32,
                chips: 40 + (self.straight * 30) as u32,
            },
            HandType::Flush => ChipsAndMult {
                mult: 4 + (self.flush * 2) as u32,
                chips: 35 + (self.flush * 15) as u32,
            },
            HandType::FullHouse => ChipsAndMult {
                mult: 4 + (self.full_house * 2) as u32,
                chips: 40 + (self.full_house * 25) as u32,
            },
            HandType::FourOfAKind => ChipsAndMult {
                mult: 7 + (self.four_of_a_kind * 3) as u32,
                chips: 60 + (self.four_of_a_kind * 30) as u32,
            },
            HandType::StraightFlush => ChipsAndMult {
                mult: 8 + (self.straight_flush * 4) as u32,
                chips: 100 + (self.straight_flush * 40) as u32,
            },
            HandType::RoyalFlush => ChipsAndMult {
                mult: 8 + (self.straight_flush * 4) as u32,
                chips: 100 + (self.straight_flush * 40) as u32,
            },
            _ => ChipsAndMult { mult: 0, chips: 0 },
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameOptions {
    pub starting_deck: GameStartingDeck,
    pub stake: GameStake,
}

#[derive(Clone, Debug)]
pub struct GameState {
    // Options
    pub stake: GameStake,
    pub starting_deck: GameStartingDeck,

    pub hands_total: u8,
    pub hand_size: u8,
    pub discards_total: u8,
    pub interest_cap: u8,
    pub joker_slots: u8,
    pub consumable_slots: u8,
    pub hand_values: HandTypeLevel,

    pub vouchers: Vec<Voucher>,
    pub jokers: Vec<JokerCard>,
    pub remaining_deck: Vec<Card>,
    pub used_cards: Vec<Card>,
    pub consumables: Vec<Consumable>,

    pub money: u8,
    pub ante: u8,
    pub blind: Blind,
    pub boss_blind: BossBlind,

    // Playing
    pub hands: u8,
    pub discards: u8,
    pub in_hand: Vec<Card>,
    pub selected_cards: Vec<Card>,

    // Scoring
    pub chips: u32,
    pub mult: u32,
    pub score: u32,

    // Statistics
    pub total_tarot_played: u32,
}

impl GameState {
    pub fn get_chips_needed(&self) -> u32 {
        self.blind.get_min_score(self.score)
    }

    pub fn start_blind(&mut self) {
        self.hands = self.hands_total;
        self.discards = self.discards_total;
        self.score = 0;
        self.chips = 0;
        self.mult = 0;

        for _ in 0..self.hand_size {
            // Get random card from remaining deck
            let card = self.remaining_deck.choose(&mut rand::thread_rng()).unwrap();

            let card_idx = self
                .remaining_deck
                .iter()
                .position(|c| c.id == card.id)
                .unwrap();

            let card = self.remaining_deck.remove(card_idx);
            self.in_hand.push(card);
        }
    }

    pub fn advance_blind(&mut self) {
        self.blind = match self.blind {
            Blind::Small => Blind::Big,
            Blind::Big => Blind::Boss(self.boss_blind.clone()),
            Blind::Boss(_) => {
                self.ante += 1;
                self.boss_blind = BossBlind::get_rand(self.ante);
                Blind::Boss(self.boss_blind.clone())
            }
        };
    }

    pub fn select_card(&mut self, card_id: u32) {
        let card_idx = self
            .in_hand
            .iter()
            .position(|card| card.id == card_id)
            .unwrap();

        let card: Card = self.in_hand.remove(card_idx);
        self.selected_cards.push(card);
    }

    pub fn deselect_card(&mut self, card_id: u32) {
        let card_idx = self
            .selected_cards
            .iter()
            .position(|card| card.id == card_id)
            .unwrap();

        let card: Card = self.selected_cards.remove(card_idx);
        self.in_hand.push(card);
    }

    pub fn play_hand(&mut self) -> (HandType, Vec<&Card>) {
        if self.hands == 0 {
            panic!("No hands left");
        }

        let (hand_type, cards) = HandType::from_cards(self.selected_cards.iter().collect());

        let chips_and_mult = self.hand_values.get_value(&hand_type);

        self.chips += chips_and_mult.chips;
        self.mult += chips_and_mult.mult;

        // Score individual cards
        for card in cards.iter() {
            let mut mult = 0;
            let mut chips = card.rank.get_base_chips();
            chips += card.extra_chips;

            match card.enhancement {
                Some(CardEnhancement::Bonus) => chips += 30,
                Some(CardEnhancement::Mult) => mult += 4,
                _ => {}
            }

            self.chips += chips;
            self.mult += mult;
        }

        self.hands -= 1;

        self.score += self.chips * self.mult;

        (hand_type, cards)
    }

    pub fn discard_hand(&mut self) {
        if self.discards == 0 {
            println!("No discards left");
            return;
        }

        self.discards -= 1;
        self.selected_cards.iter().for_each(|card| {
            self.used_cards.push(card.clone());
        });
    }
}

impl GameOptions {
    pub fn create_initial_state(self) -> GameState {
        let mut deck = Vec::new();

        let mut hands = 4;
        let mut discards = 3;
        let mut money = 4;
        let mut hand_size = 8;
        let mut interest_cap = 10;
        let mut consumable_slots = 2;
        let mut joker_slots = 5;
        let mut vouchers: Vec<Voucher> = vec![];
        let mut consumables: Vec<Consumable> = vec![];
        let ante = 0;
        let mut boss_blind = BossBlind::get_rand(ante);

        for suit in CARD_SUITS.iter() {
            for rank in CARD_RANKS.iter() {
                deck.push(Card::new(rank.clone(), suit.clone()));
            }
        }

        match &self.starting_deck {
            GameStartingDeck::Red => {
                discards += 1;
            }
            GameStartingDeck::Blue => {
                hands += 1;
            }
            GameStartingDeck::Yellow => {
                money += 10;
            }
            GameStartingDeck::Green => {
                interest_cap = 0;
            }
            GameStartingDeck::Black => {
                hand_size -= 1;
                joker_slots += 1;
            }
            GameStartingDeck::Magic => {
                vouchers.push(Voucher::CrystalBall);
                consumables.push(Consumable::Tarot(TarotCard::TheFool));
                consumables.push(Consumable::Tarot(TarotCard::TheFool));
            }
            GameStartingDeck::Nebula => {
                vouchers.push(Voucher::Telescope);
                consumable_slots -= 1;
            }
            // Ghost Deck
            GameStartingDeck::Abandoned => {
                // No face cards
                deck.retain(|card| !FACE_CARDS.contains(&card.rank));
            }
            GameStartingDeck::Checkered => {
                // Only spades and hearts
                for card in deck.iter_mut() {
                    if card.suit == CardSuit::Clubs {
                        card.suit = CardSuit::Spades;
                    }

                    if card.suit == CardSuit::Diamonds {
                        card.suit = CardSuit::Hearts;
                    }
                }
            }
            GameStartingDeck::Zodiac => {
                vouchers.push(Voucher::TarotMerchant);
                vouchers.push(Voucher::PlanetMerchant);
                vouchers.push(Voucher::Overstock);
            }
            GameStartingDeck::Painted => {
                hand_size += 2;
                joker_slots -= 1;
            }

            GameStartingDeck::Erratic => {
                deck.clear();

                for _ in 0..52 {
                    let card = Card::new(CardRank::get_rand(), CardSuit::get_rand());

                    deck.push(card);
                }
            }

            _ => {}
        }

        GameState {
            hands_total: hands,
            discards_total: discards,
            hands,
            discards,
            money,
            hand_size,
            hand_values: HandTypeLevel::new(),
            interest_cap,
            consumable_slots,
            joker_slots,
            vouchers,
            consumables,

            jokers: vec![],
            stake: self.stake,
            starting_deck: self.starting_deck,
            remaining_deck: deck,
            used_cards: vec![],
            in_hand: vec![],
            selected_cards: vec![],

            ante: ante,
            blind: Blind::Small,
            boss_blind: boss_blind,

            chips: 0,
            mult: 0,
            score: 0,

            total_tarot_played: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_hand_type_full_house() {
        let cards = vec![
            Card::new(CardRank::Two, CardSuit::Spades),
            Card::new(CardRank::Two, CardSuit::Hearts),
            Card::new(CardRank::Two, CardSuit::Clubs),
            Card::new(CardRank::Ace, CardSuit::Spades),
            Card::new(CardRank::Ace, CardSuit::Hearts),
        ];
        let (hand_type, cards) = HandType::from_cards(cards.iter().collect());
        assert_eq!(hand_type, HandType::FullHouse);
    }
    #[test]
    fn test_hand_type_flush() {
        let suit = CardSuit::get_rand();
        let cards = vec![
            Card::new(CardRank::Two, CardSuit::Diamonds),
            Card::new(CardRank::Five, CardSuit::Diamonds),
            Card::new(CardRank::Queen, CardSuit::Diamonds),
            Card::new(CardRank::Ace, CardSuit::Diamonds),
            Card::new(CardRank::King, CardSuit::Diamonds),
        ];
        let (hand_type, cards) = HandType::from_cards(cards.iter().collect());
        assert_eq!(hand_type, HandType::Flush);
    }

    #[test]
    fn test_hand_type_straight() {
        let cards = vec![
            Card::new(CardRank::Two, CardSuit::Spades),
            Card::new(CardRank::Three, CardSuit::Hearts),
            Card::new(CardRank::Four, CardSuit::Clubs),
            Card::new(CardRank::Five, CardSuit::Diamonds),
            Card::new(CardRank::Six, CardSuit::Spades),
        ];
        let (hand_type, cards) = HandType::from_cards(cards.iter().collect());
        assert_eq!(hand_type, HandType::Straight);
    }

    #[test]
    fn test_hand_type_three_of_a_kind() {
        let cards = vec![
            Card::new(CardRank::Ace, CardSuit::Spades),
            Card::new(CardRank::Ace, CardSuit::Hearts),
            Card::new(CardRank::Ace, CardSuit::Clubs),
        ];
        let (hand_type, cards) = HandType::from_cards(cards.iter().collect());
        assert_eq!(hand_type, HandType::ThreeOfAKind);
    }
    #[test]
    fn test_hand_type_two_pair() {
        let cards = vec![
            Card::new(CardRank::Ace, CardSuit::Spades),
            Card::new(CardRank::Ace, CardSuit::Hearts),
            Card::new(CardRank::Two, CardSuit::Clubs),
            Card::new(CardRank::Two, CardSuit::Diamonds),
        ];
        let (hand_type, cards) = HandType::from_cards(cards.iter().collect());
        assert_eq!(hand_type, HandType::TwoPair);
    }
    #[test]
    fn test_hand_type_pair() {
        let cards = vec![
            Card::new(CardRank::Ace, CardSuit::Spades),
            Card::new(CardRank::Ace, CardSuit::Hearts),
        ];
        let (hand_type, cards) = HandType::from_cards(cards.iter().collect());
        assert_eq!(hand_type, HandType::Pair);
    }
    #[test]
    fn test_hand_type_high_card() {
        let cards = vec![Card::new(CardRank::Ace, CardSuit::Spades)];
        let (hand_type, cards) = HandType::from_cards(cards.iter().collect());
        assert_eq!(hand_type, HandType::HighCard);
    }
}
