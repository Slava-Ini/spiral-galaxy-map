use bevy::prelude::*;

pub fn spawn_center_body(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let accretion_disk_mesh = meshes.add(Sphere::new(0.2));
    let accretion_disk_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.5),
        alpha_mode: AlphaMode::Blend,
        emissive: LinearRgba::new(10.0, 10.0, 10.0, 1.0),
        ..Default::default()
    });
    let black_hole_mesh = meshes.add(Sphere::new(0.17));
    let black_hole_material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: accretion_disk_mesh,
        material: accretion_disk_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: black_hole_mesh,
        material: black_hole_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
