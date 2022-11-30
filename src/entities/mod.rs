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
    pub last_100_estimated_frames_difference: Vec<i32>,
    pub last_estimated_amount_of_frames: i32,
    pub overshoot_handler: OvershootMovementHandler<f32, f32>,
}

pub struct OvershootMovementHandler<T, Z> {
    pub current_overshoot_amount: T,
    pub max_total_overshoot: Z,
}

impl Default for UnitMoveSpeed {
    fn default() -> Self {
        Self {
            speed: 0.1,
            last_synchronised_transform: Transform::default(),
            last_ticks_with_time: Vec::new(),
            last_100_estimated_frames_difference: Vec::new(),
            last_estimated_amount_of_frames: 0,
            overshoot_handler: OvershootMovementHandler {
                current_overshoot_amount: 0.0,
                max_total_overshoot: 10.0,
            },
        }
    }
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
