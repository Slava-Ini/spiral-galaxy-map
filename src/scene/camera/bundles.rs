use bevy::prelude::*;

use crate::scene::camera::components::*;

#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub camera: Camera3dBundle,
    pub state: PanOrbitState,
    pub settings: PanOrbitSettings,
}
