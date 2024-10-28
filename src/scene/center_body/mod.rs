use bevy::prelude::*;

use crate::scene::center_body::systems::*;

mod systems;

pub struct CenterBodyPlugin;

impl Plugin for CenterBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_center_body);
    }
}
