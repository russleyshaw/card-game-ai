use card_game_ai::{
    hand::HandType, stake::GameStake, GameOptions, GamePhase, GameStartingDeck, GameState,
    HandResult,
};
use rand::prelude::IteratorRandom;
use rand::{seq::SliceRandom, Rng};

fn main() {
    let mut state = GameState::new(GameOptions {
        stake: GameStake::White,
        starting_deck: GameStartingDeck::Red,
    });

    loop {
        println!("Ante: {}", state.ante);
        println!("Chips needed: {}", state.get_score_needed());
        println!("Blind: {:?}", state.blind);
        println!("========================\n");

        // Play the round
        loop {
            state.print_state();

            let mut cards_to_play = state.in_hand.clone();

            let possible_hands = HandType::get_possible_hands(cards_to_play.iter().collect());

            // Get best hand by chips * mult
            let mut best_hand = possible_hands.iter().max_by(|a, b| {
                let (chips_a, mult_a) = a.0.get_value(&state.hand_levels);
                let (chips_b, mult_b) = b.0.get_value(&state.hand_levels);
                (chips_a * mult_a).cmp(&(chips_b * mult_b))
            });

            let best_hand = best_hand.unwrap();
            let (chips, mult) = best_hand.0.get_value(&state.hand_levels);

            let best_score = chips * mult;
            if best_score < state.get_score_needed() - state.score {
                println!("Discarding half the cards");

                // Discard half the cards
                let half_cards = state.in_hand.len() / 2;

                // select them
                let cards_to_discard = state
                    .in_hand
                    .iter()
                    .map(|card| card.id)
                    .choose_multiple(&mut rand::thread_rng(), half_cards);

                for card in cards_to_discard {
                    state.select_card(card);
                }
                state.discard_hand();

                state.print_state();
            }

            println!(
                "Best Hand: {:?} ({} x {}): {:?}\n",
                best_hand.0, chips, mult, best_hand.1
            );
            for card in best_hand.1.iter() {
                state.select_card(card.id.clone());
            }

            let result = state.play_hand();
            println!("Score: {}", state.score);
            println!("\n\n");

            if let Some(result) = result {
                if result == HandResult::Lose {
                    println!("You lost the round");
                    return;
                } else {
                    println!("You won the round");
                    break;
                }
            }

            println!("Score: {}", state.score);
            println!("Continuing...\n");
        }

        // Do the shop
        assert_eq!(state.phase, GamePhase::Shop);

        println!("Starting blind");
        state.start_blind();
    }
}
