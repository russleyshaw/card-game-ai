#[derive(Clone, Debug)]
pub enum TarotCard {
    TheFool,
    TheMagician,
    TheHighPriestess,
    TheEmpress,
    TheEmperor,
    TheHierophant,
    TheLovers,
    TheChariot,
    Justice,
    TheHermit,
    WheelOfFortune,
    Strength,
    TheHangedMan,
    Death,
    Temperance,
    TheDevil,
    TheTower,
    TheStar,
    TheMoon,
    TheSun,
    Judgement,
    TheWorld,
}

#[derive(Clone, Debug)]
pub enum PlanetCard {
    Pluto,
    Mercury,
    Uranus,
    Venus,
    Saturn,
    Jupiter,
    Earth,
    Mars,
    Neptune,

    // Secret Planets
    PlanetX,
    Ceres,
    Eris,
}

#[derive(Clone, Debug)]
pub enum SpectralCard {
    Familiar,
    Grim,
    Incantation,
    Talisman,
    Aura,
    Wraith,
    Sigil,
    Ouija,
    Ectoplasm,
    Immolate,
    Ankh,
    DejaVu,
    Hex,
    Trance,
    Medium,
    Cryptid,
    Soul,
    BlackHole,
}

#[derive(Clone, Debug)]
pub enum Consumable {
    Tarot(TarotCard),
    Planet(PlanetCard),
    Spectral(SpectralCard),
}
