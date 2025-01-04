use crate::card::CardEdition;

#[derive(Clone, Debug)]
pub enum JokerType {
    // Common
    Joker,
    Greedy,
    Lusty,
    Wrathful,
    Gluttonous,
    Jolly,
    Zany,
    Mad,
    Crazy,
    Droll,

    Sly,
    Wily,
    Clever,
    Devious,
    Crafty,
    Half,

    CreditCard,
    Banner,
    MysticSummit,
    EightBall,
    Misprint,
    RaisedFist,
    ChaosTheClown,
    ScaryFace,
    Abstract,
    DelayedGratification,
    GrosMichel,
    EvenSteven,
    OddTodd,
    Scholar,
    BusinessCard,
    SuperNova,
    RideTheBus(u8), // Current mult
    Egg,
    Runner(u8),   // Current chips
    IceCream(u8), // Chips, Starting 100, -5 each hand.
    Splash,
    Blue,

    // Uncommon
    Stencil,
    FourFingers,
    Mime,

    CeremonialDagger,
    Marble,
    Loyalty(u8), // Countdown, 6-0
    Dusk,
    Fibonacci,
    Steel,
    Hack,
    Pareidolia,
    Space,
    Burglar,
    Blackboard,
    SixthSense,
    Constellation,
    Hiker,

    // Rare
    Dna, // Legendary
}

#[derive(Clone, Debug)]
pub struct JokerCard {
    pub joker: JokerType,
    pub value: u8,
    pub edition: CardEdition,
}
