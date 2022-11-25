use bevy::prelude::{Component, Transform, Vec3};
use bevy::time::Timer;

use crate::{CurrentTickrate, PlayerId};

#[derive(Component)]
pub struct RangedUnit {
    pub shooting_timer: Timer,
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct MoveTarget(pub Vec3);

#[derive(Component)]
pub struct UnitMoveSpeed {
    pub speed: f32,
    pub last_synchronised_transform: Transform,
    pub last_ticks_with_time: Vec<f32>,
}

impl UnitMoveSpeed {
    pub fn get_tickrate_speed(&self, tickrate: u64) -> f32 {
        self.speed * (tickrate as f32 * 0.1)
    }
}

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
