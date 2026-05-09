use std::io::Cursor;

use image::{io::Reader as ImageReader, DynamicImage};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "patterns/"]
struct Asset;

/// Represents a pattern for a Minecraft banner.
pub enum Pattern {
    Base,
    BaseDexterCanton,
    BaseGradient,
    BaseIndented,
    BaseSinisterCanton,
    Bend,
    BendSinister,
    Bordure,
    BordureIndented,
    Chevron,
    Chief,
    ChiefDexterCanton,
    ChiefIndented,
    ChiefSinisterCanton,
    CreeperCharge,
    Cross,
    Fess,
    FieldMasoned,
    Flow,
    FlowerCharge,
    Globe,
    Gradient,
    Guster,
    InvertedChevron,
    Lozenge,
    Pale,
    PaleDexter,
    PaleSinister,
    Paly,
    PerBend,
    PerBendInverted,
    PerBendSinister,
    PerBendSinisterInverted,
    PerFess,
    PerFessInverted,
    PerPale,
    PerPaleInverted,
    Rounded,
    Saltire,
    SkullCharge,
    Snout,
    Thing,
}

impl Pattern {
    fn asset_name(&self) -> String {
        match self {
            Pattern::Base => "base.png",
            Pattern::BaseDexterCanton => "base_dexter_canton.png",
            Pattern::BaseGradient => "base_gradient.png",
            Pattern::BaseIndented => "base_indented.png",
            Pattern::BaseSinisterCanton => "base_sinister_canton.png",
            Pattern::Bend => "bend.png",
            Pattern::BendSinister => "bend_sinister.png",
            Pattern::Bordure => "bordure.png",
            Pattern::BordureIndented => "bordure_indented.png",
            Pattern::Chevron => "chevron.png",
            Pattern::Chief => "chief.png",
            Pattern::ChiefDexterCanton => "chief_dexter_canton.png",
            Pattern::ChiefIndented => "chief_indented.png",
            Pattern::ChiefSinisterCanton => "chief_sinister_canton.png",
            Pattern::CreeperCharge => "creeper_charge.png",
            Pattern::Cross => "cross.png",
            Pattern::Fess => "fess.png",
            Pattern::FieldMasoned => "field_masoned.png",
            Pattern::Flow => "flow.png",
            Pattern::FlowerCharge => "flower_charge.png",
            Pattern::Globe => "globe.png",
            Pattern::Gradient => "gradient.png",
            Pattern::Guster => "guster.png",
            Pattern::InvertedChevron => "inverted_chevron.png",
            Pattern::Lozenge => "lozenge.png",
            Pattern::Pale => "pale.png",
            Pattern::PaleDexter => "pale_dexter.png",
            Pattern::PaleSinister => "pale_sinister.png",
            Pattern::Paly => "paly.png",
            Pattern::PerBend => "per_bend.png",
            Pattern::PerBendInverted => "per_bend_inverted.png",
            Pattern::PerBendSinister => "per_bend_sinister.png",
            Pattern::PerBendSinisterInverted => "per_bend_sinister_inverted.png",
            Pattern::PerFess => "per_fess.png",
            Pattern::PerFessInverted => "per_fess_inverted.png",
            Pattern::PerPale => "per_pale.png",
            Pattern::PerPaleInverted => "per_pale_inverted.png",
            Pattern::Rounded => "rounded.png",
            Pattern::Saltire => "saltire.png",
            Pattern::SkullCharge => "skull_charge.png",
            Pattern::Snout => "snout.png",
            Pattern::Thing => "thing.png",
        }
        .to_string()
    }

    pub fn image(&self) -> DynamicImage {  
        let path = self.asset_name();
        let image = Asset::get(&path).unwrap();
        let mut reader = ImageReader::new(Cursor::new(image.data));
        reader.set_format(image::ImageFormat::Png);
        reader.decode().unwrap()
    }
}