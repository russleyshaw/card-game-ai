use std::collections::HashMap;

use itertools::Itertools;

use crate::card::{Card, CardRank, CardSuit};

pub type HandLevels = HashMap<HandType, u8>;

pub struct ChipsAndMult {
    pub mult: u32,
    pub chips: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    pub fn get_value(&self, levels: &HandLevels) -> (u32, u32) {
        let level = levels.get(self).unwrap_or(&0);

        match self {
            HandType::HighCard => (5 + (level * 5) as u32, 1 + (level * 1) as u32),
            HandType::Pair => (10 + (level * 15) as u32, 2 + (level * 1) as u32),
            HandType::TwoPair => (20 + (level * 20) as u32, 2 + (level * 1) as u32),
            HandType::ThreeOfAKind => (30 + (level * 20) as u32, 3 + (level * 2) as u32),
            HandType::Straight => (40 + (level * 30) as u32, 4 + (level * 3) as u32),
            HandType::Flush => (35 + (level * 15) as u32, 4 + (level * 2) as u32),
            HandType::FullHouse => (40 + (level * 25) as u32, 4 + (level * 2) as u32),
            HandType::FourOfAKind => (60 + (level * 30) as u32, 7 + (level * 3) as u32),
            HandType::StraightFlush => (100 + (level * 40) as u32, 8 + (level * 4) as u32),
            HandType::RoyalFlush => (100 + (level * 40) as u32, 8 + (level * 4) as u32),
            _ => (0, 0),
        }
    }

    pub fn get_possible_hands(cards: Vec<&Card>) -> Vec<(HandType, Vec<&Card>)> {
        let mut possible_hands: Vec<(HandType, Vec<&Card>)> = vec![];

        // sorted_by_rank
        let mut sorted_by_rank = cards.clone();
        sorted_by_rank.sort_by(|a, b| b.rank.cmp(&a.rank));

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

        let pairs = by_ranks
            .iter()
            .filter(|(_, cards)| cards.len() == 2)
            .map(|(_, cards)| cards)
            .collect::<Vec<_>>();

        let triples = by_ranks
            .iter()
            .filter(|(_, cards)| cards.len() == 3)
            .map(|(_, cards)| cards)
            .collect::<Vec<_>>();

        // Flushes
        let flushes = by_suits
            .iter()
            .filter(|(_, cards)| cards.len() == 5)
            .map(|(_, cards)| cards)
            .collect::<Vec<_>>();

        // Flushes
        for flush in flushes {
            possible_hands.push((HandType::Flush, flush.clone()));
        }

        // Straights
        let straight_cards_needed = 5;
        if sorted_by_rank.len() >= straight_cards_needed {
            let mut straight_cards = vec![];

            for i in 0..sorted_by_rank.len() {
                straight_cards = vec![sorted_by_rank[i]];

                for j in i + 1..straight_cards_needed {
                    let prev_card = sorted_by_rank[j - 1];
                    let curr_card = sorted_by_rank[j];

                    if curr_card.rank == prev_card.rank.get_straight_prev() {
                        straight_cards.push(sorted_by_rank[j]);
                    }
                }

                if straight_cards.len() == straight_cards_needed {
                    possible_hands.push((HandType::Straight, straight_cards.clone()));
                }
            }
        }

        // Three of a kind
        for triple in triples {
            possible_hands.push((HandType::ThreeOfAKind, triple.clone()));
        }

        // Two Pair
        let two_pairs = pairs.iter().combinations(2);
        for two_pair in two_pairs {
            let mut two_pair_cards = two_pair
                .iter()
                .map(|card| **card)
                .flatten()
                .map(|card| *card)
                .collect::<Vec<_>>();

            possible_hands.push((HandType::TwoPair, two_pair_cards));
        }

        // Pair
        for pair in pairs {
            possible_hands.push((HandType::Pair, pair.clone()));
        }

        // High card
        for card in cards {
            possible_hands.push((HandType::HighCard, vec![card]));
        }

        return possible_hands;
    }

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
