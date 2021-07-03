use bevy::prelude::*;

use super::camera;

fn add_plane(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) -> bevy::prelude::PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    }
}



pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    println!("Starting The Sapphire Star ...");

    let camera = camera::camera();
    commands.spawn_bundle(camera);

    commands.spawn_bundle(add_plane(&mut meshes, &mut materials));
   
}
