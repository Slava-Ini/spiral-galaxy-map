use bevy::prelude::*;

use camera::CameraPlugin;
use fps::FpsPlugin;
use orbiting_body::OrbitingBodyPlugin;
use center_body::CenterBodyPlugin;

pub mod camera;
pub mod center_body;
pub mod fps;
pub mod orbiting_body;

pub use orbiting_body::components::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FpsPlugin,
            CameraPlugin,
            OrbitingBodyPlugin,
            CenterBodyPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK));
    }
}
