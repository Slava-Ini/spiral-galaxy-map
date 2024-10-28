use bevy::prelude::*;

use crate::scene::camera::systems::*;
use crate::scene::camera::components::*;

pub mod systems;
pub mod components;
pub mod bundles;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            pan_orbit_camera.run_if(any_with_component::<PanOrbitState>),
        );
    }
}
