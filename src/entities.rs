pub use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct MoveY;

#[derive(Component)]
pub struct Car {
    pub column: usize,
}

#[derive(Default)]
pub struct CollisionEvent;
