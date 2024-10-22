use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::camera::{pan_orbit_camera, PanOrbitCameraBundle, PanOrbitState};
use crate::fps::FpsPlugin;

mod camera;
mod fps;

const NUM_ORBITS: usize = 100;
const STARS_PER_ORBIT: usize = 100;

#[derive(Resource)]
struct OrbitSettings {
    semi_major_axis: f32,
    semi_minor_axis: f32,
    speed: f32,
}

#[derive(Component)]
struct OrbitingBody {
    angle: f32,
    orbit_speed: f32,
    semi_major_axis: f32,
    semi_minor_axis: f32,
    z_rotation: Quat,
}

enum Colors {
    Green,
    Gray,
}

impl Into<Color> for Colors {
    fn into(self) -> Color {
        match self {
            Colors::Green => Color::srgb(0.0, 1.0, 0.0),
            Colors::Gray => Color::srgb(0.5, 0.5, 0.5),
        }
    }
}

fn main() {
    let mut app = App::new();

    let startup_systems = (setup, spawn_camera);
    let update_systems = (
        pan_orbit_camera.run_if(any_with_component::<PanOrbitState>),
        orbit_system,
    );

    app.add_plugins((DefaultPlugins, FpsPlugin))
        .insert_resource(OrbitSettings {
            semi_major_axis: 0.5,
            semi_minor_axis: 0.4,
            speed: 1.0,
        })
        .add_systems(Startup, startup_systems)
        .add_systems(Update, update_systems)
        .run();
}

/// A simple system to spawn our camera
pub fn spawn_camera(mut commands: Commands) {
    let mut camera = PanOrbitCameraBundle::default();

    // Position our camera using our component,
    // not Transform (it would get overwritten)
    camera.state.center = Vec3::new(1.0, 2.0, 3.0);
    camera.state.radius = 50.0;
    camera.state.pitch = 15.0f32.to_radians();
    camera.state.yaw = 30.0f32.to_radians();

    commands.spawn(camera);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    orbit_settings: Res<OrbitSettings>,
) {
    // Spawn black hole center
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(0.5)),
        material: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    let star_material = materials.add(StandardMaterial {
        base_color: Colors::Gray.into(),
        emissive: LinearRgba::new(0.5, 0.5, 0.5, 0.5),
        ..default()
    });

    let star_mesh = PbrBundle {
        mesh: meshes.add(Sphere::new(0.07)),
        material: star_material,
        ..default()
    };

    let mut rng = rand::thread_rng();

    // Parameters to control the spiral size growth
    let major_axis_step = 0.5;
    let minor_axis_step = 0.3;
    let angle_step = PI / 16.0;

    for i in 0..NUM_ORBITS {
        let semi_major_axis = orbit_settings.semi_major_axis + i as f32 * major_axis_step;
        let semi_minor_axis = orbit_settings.semi_minor_axis + i as f32 * minor_axis_step;

        let angle_offset = i as f32 * 0.2;

        for j in 0..STARS_PER_ORBIT {
            let angle = j as f32 * angle_step + angle_offset;

            commands.spawn((
                star_mesh.clone(),
                OrbitingBody {
                    angle,
                    orbit_speed: 0.1 + rng.gen_range(0.0..0.2),
                    semi_major_axis,
                    semi_minor_axis,
                    z_rotation: Quat::from_rotation_z(angle_offset),
                },
            ));
        }
    }
}

// System to move stars along elliptical orbits
fn orbit_system(time: Res<Time>, mut query: Query<(&mut OrbitingBody, &mut Transform)>) {
    for (mut orbiting_body, mut transform) in &mut query {
        orbiting_body.angle -= orbiting_body.orbit_speed * time.delta_seconds();

        let x = orbiting_body.semi_major_axis * orbiting_body.angle.cos();
        let y = orbiting_body.semi_minor_axis * orbiting_body.angle.sin();

        // Position in local orbit coordinates (X-Y plane)
        let local_position = Vec3::new(x, y, 0.0);

        // Apply only the Z-axis rotation to keep it flat in the X-Y plane
        let rotated_position = orbiting_body.z_rotation * local_position;

        transform.translation = rotated_position;
    }
}
