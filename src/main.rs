use balatro_ai::{GameOptions, GameStake, GameStartingDeck};
use rand::seq::SliceRandom;

fn main() {
    let options = GameOptions {
        stake: GameStake::White,
        starting_deck: GameStartingDeck::Red,
    };

    let mut state = options.create_initial_state();

    state.start_blind();

    println!("Blind: {:?}", state.blind);
    println!("Chips needed: {}", state.get_chips_needed());

    println!("In hand: {:?}", state.in_hand);

    let mut cards_to_play = state.in_hand.clone();
    cards_to_play.shuffle(&mut rand::thread_rng());
    cards_to_play.truncate(5);

    for card in cards_to_play.iter() {
        state.select_card(card.id.clone());
    }
    println!("Playing: {:?}", cards_to_play);

    let (hand_type, cards) = state.play_hand();

    println!("Hand type: {:?}", hand_type);
    println!("Cards: {:?}", cards);
    println!("Score: {}", state.score);
}
