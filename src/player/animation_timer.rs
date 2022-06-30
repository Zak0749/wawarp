use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }

    pub fn just_finished(&self) -> bool {
        self.0.just_finished()
    }

    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
}
