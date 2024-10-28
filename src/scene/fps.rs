use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        let style = TextStyle {
            font_size: 20.0,
            color: Color::srgb(0.0, 1.0, 0.0),
            font: default(),
        };

        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig { text_config: style },
        });
    }
}
