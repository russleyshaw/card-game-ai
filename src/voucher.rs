// https://balatrogame.fandom.com/wiki/Vouchers

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Voucher {
    Overstock,
    OverstockPlus,

    ClearanceSale,
    Liquidation,

    Hone,
    GlowUp,

    RerollSurplus,
    RerollGlut,

    Telescope,
    OmenGlobe,

    CrystalBall,
    Observatory,

    Grabber,
    NachoTong,

    Wasteful,
    Recyclomancy,

    TarotMerchant,
    TarotTycoon,

    PlanetMerchant,
    PlanetTycoon,

    SeedMoney,
    MoneyTree,

    Blank,
    Antimatter,

    MagicTrick,
    Illusion,

    Hieroglyph,
    Petroglyph,

    DirectorsCut,
    Retcon,

    PaintBrush,
    Palette,
}

pub const BASE_VOUCHERS: [Voucher; 16] = [
    Voucher::Overstock,
    Voucher::ClearanceSale,
    Voucher::Hone,
    Voucher::RerollSurplus,
    Voucher::Telescope,
    Voucher::CrystalBall,
    Voucher::Grabber,
    Voucher::Wasteful,
    Voucher::TarotMerchant,
    Voucher::PlanetMerchant,
    Voucher::SeedMoney,
    Voucher::Blank,
    Voucher::MagicTrick,
    Voucher::Hieroglyph,
    Voucher::DirectorsCut,
    Voucher::PaintBrush,
];

pub const UPGRADED_VOUCHERS: [Voucher; 16] = [
    Voucher::OverstockPlus,
    Voucher::Liquidation,
    Voucher::GlowUp,
    Voucher::RerollGlut,
    Voucher::OmenGlobe,
    Voucher::Observatory,
    Voucher::NachoTong,
    Voucher::Recyclomancy,
    Voucher::TarotTycoon,
    Voucher::PlanetTycoon,
    Voucher::MoneyTree,
    Voucher::Antimatter,
    Voucher::Illusion,
    Voucher::Petroglyph,
    Voucher::Retcon,
    Voucher::Palette,
];
