use core::fmt;
use std::fmt::Debug;

use colored::Colorize;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BossBlind {
    // Boss
    Hook,
    Ox,
    House,
    Wall,
    Wheel,
    Arm,
    Club,
    Fish,
    Psychic,
    Goad,
    Water,
    Window,
    Manacle,
    Eye,
    Mouth,
    Plant,
    Serpent,
    Pillar,
    Needle,
    Head,
    Tooth,
    Flint,
    Mark,

    // Finisher
    AmberAcorn,
    VerdantLeaf,
    VioletVessel,
    CrimsonHeart,
    CeruleanBell,
}

impl BossBlind {
    pub fn get_rand(ante: u8) -> Self {
        let mut rng = thread_rng();

        if ante % 8 == 0 {
            return FINISHER_BLINDS.choose(&mut rng).unwrap().clone();
        }

        let available_boss_blinds = BOSS_BLINDS
            .iter()
            .filter(|blind| blind.get_min_ante() <= ante)
            .collect::<Vec<_>>();

        available_boss_blinds.choose(&mut rng).unwrap().clone()
    }

    pub fn get_reward(&self) -> u32 {
        match self {
            BossBlind::Hook => 5,
            BossBlind::Ox => 5,
            BossBlind::House => 5,
            BossBlind::Wall => 5,
            BossBlind::Wheel => 5,
            BossBlind::Arm => 5,
            BossBlind::Club => 5,
            BossBlind::Fish => 5,
            BossBlind::Psychic => 5,
            BossBlind::Goad => 5,
            BossBlind::Water => 5,
            BossBlind::Window => 5,
            BossBlind::Manacle => 5,
            BossBlind::Eye => 5,
            BossBlind::Mouth => 5,
            BossBlind::Plant => 5,
            BossBlind::Serpent => 5,
            BossBlind::Pillar => 5,
            BossBlind::Needle => 5,
            BossBlind::Head => 5,
            BossBlind::Tooth => 5,
            BossBlind::Flint => 5,
            BossBlind::Mark => 5,

            BossBlind::AmberAcorn => 8,
            BossBlind::VerdantLeaf => 8,
            BossBlind::VioletVessel => 8,
            BossBlind::CrimsonHeart => 8,
            BossBlind::CeruleanBell => 8,
        }
    }

    pub fn get_min_score(&self, base_score: u32) -> u32 {
        match self {
            BossBlind::Hook => base_score * 2,
            BossBlind::Ox => base_score * 2,
            BossBlind::House => base_score * 2,
            BossBlind::Wall => base_score * 4, // x4
            BossBlind::Wheel => base_score * 2,
            BossBlind::Arm => base_score * 2,
            BossBlind::Club => base_score * 2,
            BossBlind::Fish => base_score * 2,
            BossBlind::Psychic => base_score * 2,
            BossBlind::Goad => base_score * 2,
            BossBlind::Water => base_score * 2,
            BossBlind::Window => base_score * 2,
            BossBlind::Manacle => base_score * 2,
            BossBlind::Eye => base_score * 2,
            BossBlind::Mouth => base_score * 2,
            BossBlind::Plant => base_score * 2,
            BossBlind::Serpent => base_score * 2,
            BossBlind::Pillar => base_score * 2,
            BossBlind::Needle => base_score * 1, // x1
            BossBlind::Head => base_score * 2,
            BossBlind::Tooth => base_score * 2,
            BossBlind::Flint => base_score * 2,
            BossBlind::Mark => base_score * 2,

            BossBlind::AmberAcorn => base_score * 2,
            BossBlind::VerdantLeaf => base_score * 2,
            BossBlind::VioletVessel => base_score * 6, // x6
            BossBlind::CrimsonHeart => base_score * 2,
            BossBlind::CeruleanBell => base_score * 2,
        }
    }

    pub fn get_min_ante(&self) -> u8 {
        match self {
            BossBlind::Hook => 0,
            BossBlind::Ox => 5,
            BossBlind::House => 2,
            BossBlind::Wall => 2,
            BossBlind::Wheel => 2,
            BossBlind::Arm => 2,
            BossBlind::Club => 1,
            BossBlind::Fish => 2,
            BossBlind::Psychic => 1,
            BossBlind::Goad => 1,
            BossBlind::Water => 2,
            BossBlind::Window => 1,
            BossBlind::Manacle => 1,
            BossBlind::Eye => 2,
            BossBlind::Mouth => 2,
            BossBlind::Plant => 3,
            BossBlind::Serpent => 4,
            BossBlind::Pillar => 0,
            BossBlind::Needle => 2,
            BossBlind::Head => 0,
            BossBlind::Tooth => 2,
            BossBlind::Flint => 1,
            BossBlind::Mark => 1,

            BossBlind::AmberAcorn => 7,
            BossBlind::VerdantLeaf => 7,
            BossBlind::VioletVessel => 7,
            BossBlind::CrimsonHeart => 7,
            BossBlind::CeruleanBell => 7,
        }
    }
}

pub const BOSS_BLINDS: [BossBlind; 23] = [
    BossBlind::Hook,
    BossBlind::Ox,
    BossBlind::House,
    BossBlind::Wall,
    BossBlind::Wheel,
    BossBlind::Arm,
    BossBlind::Club,
    BossBlind::Fish,
    BossBlind::Psychic,
    BossBlind::Goad,
    BossBlind::Water,
    BossBlind::Window,
    BossBlind::Manacle,
    BossBlind::Eye,
    BossBlind::Mouth,
    BossBlind::Plant,
    BossBlind::Serpent,
    BossBlind::Pillar,
    BossBlind::Needle,
    BossBlind::Head,
    BossBlind::Tooth,
    BossBlind::Flint,
    BossBlind::Mark,
];

pub const FINISHER_BLINDS: [BossBlind; 5] = [
    BossBlind::AmberAcorn,
    BossBlind::VerdantLeaf,
    BossBlind::VioletVessel,
    BossBlind::CrimsonHeart,
    BossBlind::CeruleanBell,
];

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Blind {
    Small,
    Big,
    Boss(BossBlind),
}

impl Blind {
    pub fn get_min_ante(&self) -> u8 {
        match self {
            Blind::Small => 0,
            Blind::Big => 0,
            Blind::Boss(boss) => boss.get_min_ante(),
        }
    }

    pub fn get_min_score(&self, base_score: u32) -> u32 {
        match self {
            Blind::Small => base_score * 1,
            Blind::Big => base_score * 2,
            Blind::Boss(boss) => boss.get_min_score(base_score),
        }
    }

    pub fn get_reward(&self) -> u32 {
        match self {
            Blind::Small => 3,
            Blind::Big => 4,
            Blind::Boss(boss) => boss.get_reward(),
        }
    }
}

impl Debug for Blind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Blind::Small => "Small".green(),
            Blind::Big => "Big".yellow(),
            Blind::Boss(boss) => format!("{:?}", boss).red(),
        };

        write!(f, "{}", txt)
    }
}
