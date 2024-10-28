use bevy::prelude::*;

use crate::configuration::resources::Configuration;
use systems::*;

pub mod components;
pub mod systems;

pub struct OrbitingBodyPlugin;


impl Plugin for OrbitingBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bodies).add_systems(
            Update,
            (
                orbit_bodies,
                // -- Note --
                // Not a great solution, can be vastly improved
                // by de-spawning and changing certain entities
                // Omitting for now
                respawn_bodies
                    .run_if(resource_changed::<Configuration>),
            ),
        );
    }
}
