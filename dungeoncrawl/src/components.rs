use crate::prelude::*;

// A component that denotes an enemy
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

// A component that denotes a player
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

// A component that denotes the quality of moving randomly
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

// A component that denotes a renderable entity
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}
