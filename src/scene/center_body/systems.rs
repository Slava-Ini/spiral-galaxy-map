use bevy::prelude::*;

pub fn spawn_center_body(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(0.1));
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::new(100.0, 100.0, 100.0, 1.0),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
