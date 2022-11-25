
use bevy::prelude::{Component, Vec3};
use bevy::time::Timer;

use crate::PlayerId;

#[derive(Component)]
pub struct RangedUnit {
    pub shooting_timer: Timer,
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct MoveTarget(pub Vec3);

#[derive(Component)]
pub struct UnitMoveSpeed(pub f32);

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct OtherPlayerControlled(pub PlayerId);

#[derive(Component)]
pub struct OtherPlayerCamera(pub PlayerId);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Neutral;

#[derive(Component)]
pub struct Friendly;
