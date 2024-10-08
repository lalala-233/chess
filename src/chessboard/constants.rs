use bevy::color::*;

pub const BLACK: Srgba = Srgba::rgb(0.717_647_1, 0.752_941_2, 0.847_058_83);
pub const WHITE: Srgba = Srgba::rgb(0.909_803_9, 0.929_411_77, 0.976_470_6);
pub const BLUE: Srgba = Srgba::new(0.482_352_94, 0.380_392_16, 1., 0.8);

pub const DARK_BLACK: Srgba = Srgba::rgb(0., 0., 0.);

pub const GRAY: Srgba = Srgba::new(0.62, 0.62, 0.62, 1.);
pub const GREEN: Srgba = Srgba::new(0.376, 0.922, 0.212, 1.);
pub const TRANSPARENT_PURPLE: Srgba = Srgba::new(0.524, 0., 0.524, 0.7);

pub const SQUARE_SIZE: f32 = 64.;

pub const LEFT: f32 = -SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;
pub const BOTTOM: f32 = -SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;

pub const PIECES_CODE: [&str; 12] = [
    "wp", "wr", "wn", "wb", "wq", "wk", "bp", "br", "bn", "bb", "bq", "bk",
];
