use crate::prelude::*;

// A component that denotes an enemy
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enemy;

// A component that denotes health for an entity
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

// A component that denotes the name of an entity
#[derive(Clone, PartialEq, Eq)]
pub struct Name(pub String);

// A component that denotes a player
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player;

// A component that denotes the quality of moving randomly
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MovingRandomly;

// A component that denotes a renderable entity
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

// A component that signals the intention of an entity to attack
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

// A component that signals the intention of an entity to move
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
