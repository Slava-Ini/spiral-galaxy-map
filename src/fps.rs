use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;

use crate::Colors;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        let style = TextStyle {
            font_size: 20.0,
            color: Colors::Green.into(),
            font: default(),
        };

        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig { text_config: style },
        });
    }
}
