pub mod ante;
pub mod blind;
pub mod booster;
pub mod card;
pub mod consumable;
pub mod hand;
pub mod joker;
pub mod stake;
mod utils;
pub mod voucher;

use std::{
    clone,
    collections::HashMap,
    fmt::Debug,
    sync::atomic::{AtomicU32, Ordering},
};

use ante::get_base_score;
use blind::{Blind, BossBlind};
use card::{
    debug_cards, Card, CardEnhancement, CardRank, CardSuit, CARD_RANKS, CARD_SUITS, FACE_CARDS,
};
use consumable::{Consumable, TarotCard};
use hand::HandType;
use itertools::Itertools;
use joker::JokerCard;
use rand::seq::SliceRandom;
use stake::GameStake;
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
pub enum GamePhase {
    Playing,
    Shop,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandResult {
    Win,
    Lose,
}

#[derive(Clone, Debug)]
pub struct ChipsAndMult {
    pub chips: u32,
    pub mult: u32,
}

#[derive(Clone, Debug)]
pub struct GameOptions {
    pub starting_deck: GameStartingDeck,
    pub stake: GameStake,
}

pub type PlayerMoney = i32;

enum BoosterPack {
    Standard,
    StandardJumbo,
    StandardMega,

    Arcana,
    JumboArcana,
    MegaArcana,

    Celestial,
    JumboCelestial,
    MegaCelestial,

    Buffoon,
    JumboBuffoon,
    MegaBuffoon,

    Spectral,
    JumboSpectral,
    MegaSpectral,
}

impl BoosterPack {
    pub fn get_cost(&self) -> u8 {
        match self {
            BoosterPack::Standard => 4,
            BoosterPack::StandardJumbo => 6,
            BoosterPack::StandardMega => 8,

            BoosterPack::Arcana => 4,
            BoosterPack::JumboArcana => 6,
            BoosterPack::MegaArcana => 8,

            BoosterPack::Celestial => 4,
            BoosterPack::JumboCelestial => 6,
            BoosterPack::MegaCelestial => 8,

            BoosterPack::Buffoon => 4,
            BoosterPack::JumboBuffoon => 6,
            BoosterPack::MegaBuffoon => 8,

            BoosterPack::Spectral => 4,
            BoosterPack::JumboSpectral => 6,
            BoosterPack::MegaSpectral => 8,
        }
    }

    pub fn get_weight(&self) -> f32 {
        match self {
            BoosterPack::Standard => 4.0,
            BoosterPack::StandardJumbo => 2.0,
            BoosterPack::StandardMega => 0.5,

            BoosterPack::Arcana => 4.0,
            BoosterPack::JumboArcana => 2.0,
            BoosterPack::MegaArcana => 0.5,

            BoosterPack::Celestial => 4.0,
            BoosterPack::JumboCelestial => 2.0,
            BoosterPack::MegaCelestial => 0.5,

            BoosterPack::Buffoon => 1.2,
            BoosterPack::JumboBuffoon => 0.6,
            BoosterPack::MegaBuffoon => 0.15,

            BoosterPack::Spectral => 0.6,
            BoosterPack::JumboSpectral => 0.3,
            BoosterPack::MegaSpectral => 0.07,
        }
    }
}

enum SaleCard {
    Joker(JokerCard),
    Consumable(Consumable),
}

pub struct GameShopState {
    pub cards: Vec<SaleCard>,

    pub vouchers: Vec<Voucher>,
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
    pub hand_levels: HashMap<HandType, u8>,

    pub vouchers: Vec<Voucher>,
    pub jokers: Vec<JokerCard>,
    pub remaining_deck: Vec<Card>,
    pub used_cards: Vec<Card>,
    pub consumables: Vec<Consumable>,

    pub money: u8,
    pub ante: u8,
    pub blind: Blind,
    pub boss_blind: BossBlind,

    pub phase: GamePhase,

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
    pub fn new(options: GameOptions) -> Self {
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
        let ante = 1;
        let mut boss_blind = BossBlind::get_rand(ante);

        for suit in CARD_SUITS.iter() {
            for rank in CARD_RANKS.iter() {
                deck.push(Card::new(rank.clone(), suit.clone()));
            }
        }

        match &options.starting_deck {
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

        let mut state = GameState {
            hands_total: hands,
            discards_total: discards,
            hands,
            discards,
            money,
            hand_size,
            hand_levels: HashMap::new(),
            interest_cap,
            consumable_slots,
            joker_slots,
            vouchers,
            consumables,

            jokers: vec![],
            stake: options.stake,
            starting_deck: options.starting_deck,
            remaining_deck: deck,
            used_cards: vec![],
            in_hand: vec![],
            selected_cards: vec![],

            phase: GamePhase::Playing,

            ante: ante,
            blind: Blind::Small,
            boss_blind: boss_blind,

            chips: 0,
            mult: 0,
            score: 0,

            total_tarot_played: 0,
        };

        state.init_blind();
        return state;
    }

    pub fn get_score_needed(&self) -> u32 {
        let base_chips = get_base_score(self.ante, &self.stake);

        self.blind.get_min_score(base_chips)
    }

    pub fn print_state(&self) {
        println!("================================================");
        println!(
            "Hands: {}/{} | Discards: {}/{} | Score: {} | Money: {}",
            self.hands,
            self.hands_total,
            self.discards,
            self.discards_total,
            self.score,
            self.money
        );

        println!(
            "Ante: {} | Blind: {:?} | Boss blind: {:?}",
            self.ante, self.blind, self.boss_blind
        );

        println!("Phase: {:?}", self.phase);

        println!("Selected cards: {}", debug_cards(&self.selected_cards));
        println!("In hand: {}", debug_cards(&self.in_hand));
        println!("Used cards: {}", debug_cards(&self.used_cards));

        println!("===============================================\n");
    }

    fn init_blind(&mut self) {
        self.hands = self.hands_total;
        self.discards = self.discards_total;
        self.score = 0;
        self.chips = 0;
        self.mult = 0;

        // Put all cards back into deck
        for card in self.used_cards.iter() {
            self.remaining_deck.push(card.clone());
        }
        self.used_cards.clear();

        for card in self.in_hand.iter() {
            self.remaining_deck.push(card.clone());
        }
        self.in_hand.clear();

        for card in self.selected_cards.iter() {
            self.remaining_deck.push(card.clone());
        }
        self.selected_cards.clear();

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

    pub fn start_blind(&mut self) {
        assert_eq!(self.phase, GamePhase::Shop);
        self.phase = GamePhase::Playing;

        self.init_blind();
    }

    fn advance_blind(&mut self) {
        assert_eq!(self.phase, GamePhase::Playing);

        self.blind = match self.blind {
            Blind::Small => Blind::Big,
            Blind::Big => Blind::Boss(self.boss_blind.clone()),
            Blind::Boss(_) => {
                self.ante += 1;
                self.boss_blind = BossBlind::get_rand(self.ante);
                Blind::Boss(self.boss_blind.clone())
            }
        };

        self.phase = GamePhase::Shop;
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

    pub fn play_hand(&mut self) -> Option<HandResult> {
        if self.hands == 0 {
            return Some(HandResult::Lose);
        }

        let cards = self.selected_cards.iter().collect();

        let (hand_type, cards) = HandType::from_cards(cards);

        let chips_and_mult = hand_type.get_value(&self.hand_levels);

        self.chips += chips_and_mult.0;
        self.mult += chips_and_mult.1;

        // Score individual cards
        for card in cards.iter() {
            let mut mult = 0;
            let mut chips = card.rank.get_base_chips();
            chips += card.extra_chips;

            match card.enhancement {
                Some(CardEnhancement::Bonus) => chips += 30,
                Some(CardEnhancement::Mult) => mult += 4,
                _ => {}
            };

            self.chips += chips;
            self.mult += mult;
        }

        self.hands -= 1;
        self.score += self.chips * self.mult;

        self.chips = 0;
        self.mult = 0;

        if self.score >= self.get_score_needed() {
            self.advance_blind();
            return Some(HandResult::Win);
        }

        if self.hands == 0 && self.score < self.get_score_needed() {
            return Some(HandResult::Lose);
        }

        // Put selected cards into used cards
        self.use_selected_cards();

        // Redraw cards up to hand size
        self.fill_in_hand();
        return None;
    }

    fn use_selected_cards(&mut self) {
        for card in self.selected_cards.iter() {
            self.used_cards.push(card.clone());
        }
        self.selected_cards.clear();
    }

    fn fill_in_hand(&mut self) {
        let num_to_draw = self.hand_size as usize - self.in_hand.len();
        for _ in 0..num_to_draw {
            let card = self.remaining_deck.choose(&mut rand::thread_rng()).unwrap();
            self.in_hand.push(card.clone());
        }
    }

    pub fn discard_hand(&mut self) {
        assert_ne!(self.discards, 0);

        self.discards -= 1;
        self.selected_cards.iter().for_each(|card| {
            self.used_cards.push(card.clone());
        });

        self.selected_cards.clear();
        self.fill_in_hand();
    }
}
