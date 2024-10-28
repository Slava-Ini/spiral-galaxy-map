use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanOrbitAction {
    Pan,
    Orbit,
    Zoom,
}

#[derive(Component)]
pub struct PanOrbitState {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Component)]
pub struct PanOrbitSettings {
    /// World units per pixel of mouse motion
    pub pan_sensitivity: f32,
    /// Radians per pixel of mouse motion
    pub orbit_sensitivity: f32,
    /// Exponent per pixel of mouse motion
    pub zoom_sensitivity: f32,
    /// Mouse click to hold for panning
    pub pan_key: Option<MouseButton>,
    /// Mouse click to hold for orbiting
    pub orbit_key: Option<MouseButton>,
    /// Key to hold for zooming
    pub zoom_key: Option<KeyCode>,
    /// What action is bound to the scroll wheel?
    pub scroll_action: Option<PanOrbitAction>,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
}


impl Default for PanOrbitState {
    fn default() -> Self {
        PanOrbitState {
            center: Vec3::ZERO,
            radius: 1.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl Default for PanOrbitSettings {
    fn default() -> Self {
        PanOrbitSettings {
            pan_sensitivity: 0.001,                 
            orbit_sensitivity: 0.1f32.to_radians(), 
            zoom_sensitivity: 0.01,
            pan_key: Some(MouseButton::Left),
            orbit_key: Some(MouseButton::Right),
            zoom_key: Some(KeyCode::ShiftLeft),
            scroll_action: Some(PanOrbitAction::Zoom),
            scroll_line_sensitivity: 16.0, 
            scroll_pixel_sensitivity: 1.0,
        }
    }
}
