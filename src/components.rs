pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    // A helper class stores both fg-bg color.
    pub color: ColorPair,
    pub glyph: FontCharType,
}

// Empty component is call a tag if any field excluded.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;
