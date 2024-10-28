use bevy::prelude::*;

use crate::scene::SemiAxis;
use crate::ui::components::{ButtonAction, ButtonKind};

#[derive(Resource, Clone)]
pub struct Configuration {
    pub dust: bool,
    pub h2: bool,
    pub filament: bool,
    pub star_count: u32,
    pub velocity: f32,
    pub semi_axis: SemiAxis,
    pub stars_per_orbit: u8,
    pub dimming_speed: f32,
    pub orbit_density: f32,
    pub orbit_rotation: f32,
    pub star_size: f32,
}

impl Configuration {
    pub fn map_to_button_value(&self, button_kind: ButtonKind) -> String {
        match button_kind {
            ButtonKind::StarCount => format!("{}.000", self.star_count / 1_000),
            ButtonKind::StarSize => format!("{:.2}", self.star_size),
            ButtonKind::Velocity => format!("{:.2}", self.velocity),
            ButtonKind::SemiAxis => format!("{}", self.semi_axis),
            ButtonKind::OrbitDensity => format!("{:.2}", self.orbit_density),
            ButtonKind::OrbitRotation => format!("{:.2}", self.orbit_rotation),
        }
    }

    pub fn update(&mut self, button_kind: ButtonKind, action: ButtonAction) {
        match button_kind {
            ButtonKind::StarCount => {
                self.update_star_count(action);
            }
            ButtonKind::StarSize => {
                self.update_star_size(action);
            }
            ButtonKind::Velocity => {
                self.update_velocity(action);
            }
            ButtonKind::SemiAxis => {
                self.update_semi_axis(action);
            }
            ButtonKind::OrbitDensity => {
                self.update_orbit_density(action);
            }
            ButtonKind::OrbitRotation => {
                self.update_orbit_rotation(action);
            }
        }
    }

    const STAR_COUNT_STEP: u32 = 1_000;
    const STAR_COUNT_LIMIT: (u32, u32) = (5_000, 15_000);

    fn update_star_count(&mut self, action: ButtonAction) {
        match action {
            ButtonAction::Increment if self.star_count < Self::STAR_COUNT_LIMIT.1 => {
                self.star_count += Self::STAR_COUNT_STEP;
            }
            ButtonAction::Decrement if self.star_count > Self::STAR_COUNT_LIMIT.0 => {
                self.star_count -= Self::STAR_COUNT_STEP;
            }
            _ => {}
        }
    }

    const STAR_SIZE_STEP: f32 = 0.01;
    const STAR_SIZE_LIMIT: (f32, f32) = (0.01, 0.07);

    fn update_star_size(&mut self, action: ButtonAction) {
        match action {
            ButtonAction::Increment if self.star_size < Self::STAR_SIZE_LIMIT.1 => {
                self.star_size += Self::STAR_SIZE_STEP;
            }
            ButtonAction::Decrement if self.star_size > Self::STAR_SIZE_LIMIT.0 => {
                self.star_size -= Self::STAR_SIZE_STEP;
            }
            _ => {}
        }
    }

    const VELOCITY_STEP: f32 = 0.1;
    const VELOCITY_LIMIT: (f32, f32) = (0.1, 5.0);

    fn update_velocity(&mut self, action: ButtonAction) {
        match action {
            ButtonAction::Increment if self.velocity < Self::VELOCITY_LIMIT.1 => {
                self.velocity += Self::VELOCITY_STEP
            }
            ButtonAction::Decrement if self.velocity > Self::VELOCITY_LIMIT.0 => {
                self.velocity -= Self::VELOCITY_STEP
            }
            _ => {},
        };
    }

    const SEMI_AXIS_STEP: f32 = 0.05;
    const SEMI_AXIS_LIMIT: (f32, f32) = (0.2, 1.1);

    fn update_semi_axis(&mut self, action: ButtonAction) {
        match action {
            ButtonAction::Increment if self.semi_axis.major < Self::SEMI_AXIS_LIMIT.1 => {
                self.semi_axis += Self::SEMI_AXIS_STEP;
            }
            ButtonAction::Decrement if self.semi_axis.major > Self::SEMI_AXIS_LIMIT.0 => {
                self.semi_axis -= Self::SEMI_AXIS_STEP;
            }
            _ => {}
        }
    }

    const ORBIT_DENSITY_STEP: f32 = 0.1;
    const ORBIT_DENSITY_LIMIT: (f32, f32) = (0.5, 4.0);

    fn update_orbit_density(&mut self, action: ButtonAction) {
        match action {
            ButtonAction::Increment if self.orbit_density < Self::ORBIT_DENSITY_LIMIT.1 => {
                self.orbit_density += Self::ORBIT_DENSITY_STEP;
            }
            ButtonAction::Decrement if self.orbit_density > Self::ORBIT_DENSITY_LIMIT.0 => {
                self.orbit_density -= Self::ORBIT_DENSITY_STEP;
            }
            _ => {}
        }
    }

    const ORBIT_ROTATION_STEP: f32 = 0.1;
    const ORBIT_ROTATION_LIMIT: (f32, f32) = (0.0, 1.0);

    fn update_orbit_rotation(&mut self, action: ButtonAction) {
        match action {
            ButtonAction::Increment if self.orbit_rotation < Self::ORBIT_ROTATION_LIMIT.1 => {
                self.orbit_rotation += Self::ORBIT_ROTATION_STEP;
            }
            ButtonAction::Decrement if self.orbit_rotation > Self::ORBIT_ROTATION_LIMIT.0 => {
                self.orbit_rotation -= Self::ORBIT_ROTATION_STEP;
            }
            _ => {}
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            velocity: 1.0,
            star_count: 10_000,
            dust: true,
            h2: true,
            filament: true,
            semi_axis: SemiAxis::default(),
            stars_per_orbit: 100,
            dimming_speed: 150.0,
            orbit_density: 2.0,
            orbit_rotation: 0.2,
            star_size: 0.02,
        }
    }
}
