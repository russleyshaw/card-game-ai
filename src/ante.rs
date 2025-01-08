use crate::stake::GameStake;

pub fn get_base_score(ante: u8, stake: &GameStake) -> u32 {
    let base_score: (u32, u32, u32) = match ante {
        0 => (100, 100, 100),
        1 => (300, 300, 300),
        2 => (800, 900, 1000),
        3 => (2000, 2600, 3200),
        4 => (5000, 8000, 9000),
        5 => (11000, 20000, 25000),
        6 => (20000, 36000, 60000),
        7 => (35000, 60000, 110000),
        8 => (50000, 100000, 200000),

        _ => (0, 0, 0),
    };

    match stake {
        GameStake::White => base_score.0,
        GameStake::Red => base_score.0,
        GameStake::Green => base_score.1,
        GameStake::Black => base_score.1,
        GameStake::Blue => base_score.1,
        GameStake::Purple => base_score.2,
        GameStake::Orange => base_score.2,
        GameStake::Gold => base_score.2,
    }
}
