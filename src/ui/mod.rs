use bevy::prelude::*;

use crate::{configuration::resources::Configuration, ui::systems::*};

pub mod components;
pub mod styles;
pub mod systems;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum UIState {
    #[default]
    Displayed,
    #[allow(dead_code)]
    Hidden,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UIState>()
            .add_systems(Startup, build_ui)
            // -- Note --
            // Displaying and hiding UI is not finished
            .add_systems(
                Update,
                (
                    interact_with_checkbox.run_if(in_state(UIState::Displayed)),
                    interact_with_button.run_if(in_state(UIState::Displayed)),
                    // -- Note --
                    // Not a great way to update UI by rebuilding it from scratch
                    // but more concise way required more work
                    rebuild_ui.run_if(resource_changed::<Configuration>),
                ),
            );
    }
}
