use bevy::prelude::*;

use resources::Configuration;

pub mod resources;

pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Configuration>();
    }
}
