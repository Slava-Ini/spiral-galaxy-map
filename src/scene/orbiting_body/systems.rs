use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::configuration::resources::*;
use crate::scene::*;

const ANGLE_STEP: f32 = PI / 16.0;

pub fn spawn_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    configuration: Res<Configuration>,
) {
    let Configuration {
        star_count,
        dust,
        h2,
        filament,
        velocity,
        semi_axis,
        stars_per_orbit,
        dimming_speed,
        orbit_density,
        orbit_rotation,
        star_size,
    } = *configuration;

    let mut rng = thread_rng();

    let num_orbits = star_count / stars_per_orbit as u32;

    for i in 0..num_orbits {
        // -- Stars orbits rotate and grow farther from the center
        let orbit_density = i as f32 / orbit_density;
        let semi_axis = semi_axis + orbit_density * semi_axis;
        let angle_offset = i as f32 * orbit_rotation;

        // -- Stars dimming farther from the center
        let dimming_step = dimming_speed / num_orbits as f32;
        let step_percent = dimming_step / num_orbits as f32;
        let dimming_channel = 1.0 - i as f32 * step_percent;

        // -- Stars speed decreasing farther from the center
        let orbit_speed = (velocity / (i as f32 + 1.0)) * 3.0;

        // -- Initialize meshes
        let star_mesh = get_star_mesh(&mut meshes, &mut materials, dimming_channel, star_size);
        let filament_mesh =
            get_filament_mesh(&mut meshes, &mut materials, dimming_channel, star_size);
        let dust_mesh = get_dust_mesh(&mut meshes, &mut materials, dimming_channel, star_size);
        let h2_mesh = get_h2_mesh(&mut meshes, &mut materials, dimming_channel, star_size);

        for j in 0..stars_per_orbit {
            // -- Stars are distributed uniformly in the orbit
            let angle = j as f32 * ANGLE_STEP + angle_offset;

            // -- Speed and semi-axis are slightly altered
            let semi_axis = semi_axis * rng.gen_range(0.9..1.1);
            let speed_fluctuation = rng.gen_range(0.01..0.1);

            let orbiting_body = OrbitingBody {
                angle,
                semi_axis,
                orbit_speed: orbit_speed + speed_fluctuation,
                y_rotation: Quat::from_rotation_y(angle_offset),
            };

            let mut spawn_star = true;
            let mut spawn_h2 = true;
            let mut spawn_dust = true;

            // -- Spawn filaments
            if filament && j % 4 == 0 {
                commands.spawn((filament_mesh.clone(), orbiting_body.clone()));

                spawn_star = false;
                spawn_dust = false;
                spawn_h2 = false;
            }

            // -- Spawn HII regions
            if h2 && spawn_h2 && j % 30 == 0 {
                commands.spawn((h2_mesh.clone(), orbiting_body.clone()));

                spawn_star = false;
                spawn_dust = false;
            }

            // -- Spawn dust
            if dust && spawn_dust && j % 5 == 0 {
                commands.spawn((dust_mesh.clone(), orbiting_body.clone()));

                spawn_star = false;
            }


            // -- Spawn a star
            if spawn_star {
                commands.spawn((star_mesh.clone(), orbiting_body));
            }
        }
    }
}

pub fn respawn_bodies(
    mut commands: Commands,
    configuration: Res<Configuration>,
    query: Query<Entity, With<OrbitingBody>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    if configuration.is_changed() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        spawn_bodies(commands, meshes, materials, configuration);
    }
}

pub fn orbit_bodies(time: Res<Time>, mut query: Query<(&mut OrbitingBody, &mut Transform)>) {
    for (mut orbiting_body, mut transform) in &mut query {
        orbiting_body.angle += orbiting_body.orbit_speed * time.delta_seconds();

        let x = orbiting_body.semi_axis.major * orbiting_body.angle.cos();
        let z = orbiting_body.semi_axis.minor * orbiting_body.angle.sin();

        // -- Position in local orbit coordinates (X-Z plane)
        let local_position = Vec3::new(x, 0.0, z);

        // -- Apply only the Y-axis rotation
        let rotated_position = orbiting_body.y_rotation * local_position;

        transform.translation = rotated_position;
    }
}

fn get_star_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    dimming_channel: f32,
    star_size: f32,
) -> PbrBundle {
    let d = dimming_channel;
    let material = StandardMaterial {
        base_color: Color::NONE,
        emissive: LinearRgba::new(d * 1.2, d * 1.2, d, 1.0),
        ..default()
    };

    PbrBundle {
        mesh: meshes.add(Sphere::new(star_size).mesh().uv(6, 3)),
        material: materials.add(material),
        ..default()
    }
}

fn get_filament_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    dimming_channel: f32,
    star_size: f32,
) -> PbrBundle {
    let d = dimming_channel * 0.8;
    let material = StandardMaterial {
        base_color: Color::srgba(d, d, d * 2.0, 0.1),
        alpha_mode: AlphaMode::Blend,
        ..default()
    };
    let size = star_size * 9.0;

    PbrBundle {
        mesh: meshes.add(Sphere::new(size).mesh().uv(12, 6)),
        material: materials.add(material),
        ..default()
    }
}

fn get_h2_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    dimming_channel: f32,
    star_size: f32,
) -> PbrBundle {
    let d = dimming_channel * 0.8;
    let material = StandardMaterial {
        base_color: Color::NONE,
        emissive: LinearRgba::new(d * 10.0, d, d, 0.9),
        ..default()
    };
    let size = star_size * 2.0;

    PbrBundle {
        mesh: meshes.add(Sphere::new(size).mesh().uv(6, 3)),
        material: materials.add(material),
        ..default()
    }
}

fn get_dust_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    dimming_channel: f32,
    star_size: f32,
) -> PbrBundle {
    let d = dimming_channel * 0.8;
    let material = StandardMaterial {
        base_color: Color::srgba(d, d, d, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
    };
    let size = star_size * 3.0;

    PbrBundle {
        mesh: meshes.add(Sphere::new(size).mesh().uv(6, 3)),
        material: materials.add(material),
        ..default()
    }
}
