use bevy::prelude::{Component, Resource, Transform, Vec3};
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
    pub overshoot_handler: OvershootMovementHandler<f32, f32>,
}

pub struct OvershootMovementHandler<T, Z> {
    pub current_overshoot_amount: T,
    pub max_total_overshoot: Z,
}

#[derive(Component, Resource)]
pub struct InterpolationFrameManager {
    pub last_ticks_with_time: Vec<f32>,
    pub last_100_estimated_frames_difference: Vec<i32>,
    pub last_estimated_amount_of_frames: i32,
}

impl InterpolationFrameManager {
    pub fn reset(&mut self) {
        self.last_ticks_with_time.clear();
        self.last_100_estimated_frames_difference.clear();
        self.last_estimated_amount_of_frames = 0;
    }

    pub fn add_frame(&mut self, difference: i32) {
        self.last_100_estimated_frames_difference.push(difference);
        if self.last_100_estimated_frames_difference.len() > 100 {
            self.last_100_estimated_frames_difference.remove(0);

            // resizes the vector to 100 elements
            self.last_100_estimated_frames_difference.shrink_to_fit();
        }
    }

    pub fn get_average_time_between_frames(&self) -> f32 {
        if self.last_100_estimated_frames_difference.is_empty() {
            return 0.0;
        }
        self.last_ticks_with_time.iter().sum::<f32>() / self.last_ticks_with_time.len() as f32
    }

    pub fn get_average_difference_between_estimated_and_actual_frames(&self) -> i32 {
        if self.last_100_estimated_frames_difference.len() > 0 {
            let difference_total = self.last_100_estimated_frames_difference.iter().sum::<i32>();
            difference_total / move_speed.last_100_estimated_frames_difference.len() as i32
        } else {
            0
        }
    }

    pub fn get_estimated_amount_of_frames_to_interpolate_total(&self, tickrate: u64, with_difference: bool) -> i32 {
        let estimated_frames_to_interpolate_total = ((tickrate as f32 / 1000.0 / self.get_average_time_between_frames() as f32).floor()).clamp(1.0, f32::MAX);
        return if with_difference {
            (estimated_frames_to_interpolate_total + self.get_average_difference_between_estimated_and_actual_frames() as f32).clamp(1.0, f32::MAX) as i32
        } else {
            estimated_frames_to_interpolate_total as i32
        };
    }

    pub fn get_estimated_frames_left(&self, tickrate: u64, with_difference: bool) -> i32 {
        (self.get_estimated_amount_of_frames_to_interpolate_total(tickrate, with_difference) - self.last_100_estimated_frames_difference.len() as i32).clamp(0, self.get_estimated_amount_of_frames_to_interpolate_total(tickrate, with_difference))
    }
}

impl Default for InterpolationFrameManager {
    fn default() -> Self {
        Self {
            last_ticks_with_time: Vec::new(),
            last_100_estimated_frames_difference: Vec::new(),
            last_estimated_amount_of_frames: 0,
        }
    }
}

impl Default for UnitMoveSpeed {
    fn default() -> Self {
        Self {
            speed: 0.1,
            last_synchronised_transform: Transform::default(),
            overshoot_handler: OvershootMovementHandler {
                current_overshoot_amount: 0.0,
                max_total_overshoot: 0.1,
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
