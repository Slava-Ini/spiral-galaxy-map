use bevy::prelude::*;

#[derive(Component)]
pub struct UI;

#[derive(Component)]
pub struct Checkbox {
    pub checked: bool,
    pub kind: CheckboxKind,
}

#[derive(Component)]
pub enum CheckboxKind {
    Dust,
    HII,
    Filament,
}

#[derive(Component)]
pub struct Marker; 

#[derive(Component, Clone, Copy)]
pub struct TweakButton {
    pub action: ButtonAction,
    pub kind: ButtonKind,
}

#[derive(Component, Clone, Copy)]
pub enum ButtonAction {
    Increment,
    Decrement,
}

#[derive(Component, Copy, Clone)]
pub enum ButtonKind {
    StarCount,
    StarSize,
    Velocity,
    SemiAxis,
    OrbitDensity,
    OrbitRotation,
}

impl ToString for ButtonKind {
    fn to_string(&self) -> String {
        match self {
            ButtonKind::StarCount => "Star Count".to_string(),
            ButtonKind::StarSize => "Star Size".to_string(),
            ButtonKind::Velocity => "Velocity".to_string(),
            ButtonKind::SemiAxis => "Semi Axis".to_string(),
            ButtonKind::OrbitDensity => "Orbit Density".to_string(),
            ButtonKind::OrbitRotation => "Orbit Rotation".to_string(),
        }
    }
}

