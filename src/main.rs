use bevy::prelude::*;

use configuration::ConfigurationPlugin;
use scene::ScenePlugin;
use ui::UIPlugin;

mod configuration;
mod scene;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, ScenePlugin, UIPlugin, ConfigurationPlugin))
        .run();
}
