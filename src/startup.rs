use bevy::prelude::*;

use super::camera;

fn plane(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    }
}

fn cube(
    color: [f32;3],
    coords: [f32;3],
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(color[0],color[1],color[2]).into()),
        transform: Transform::from_xyz(coords[0],coords[1],coords[2]),
        ..Default::default()
    }
}

fn light() -> LightBundle {
    LightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
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

    commands.spawn_bundle(plane(&mut meshes, &mut materials));
    commands.spawn_bundle(cube([0.8, 0.7, 0.6],
                               [1.5, 0.5, 1.5],
                               &mut meshes,
                               &mut materials));
    commands.spawn_bundle(cube([0.8, 0.7, 0.6],
                               [1.5, 0.5, -1.5],
                               &mut meshes,
                               &mut materials));
    commands.spawn_bundle(cube([0.8, 0.7, 0.6],
                               [-1.5, 0.5, 1.5],
                               &mut meshes,
                               &mut materials));
    commands.spawn_bundle(cube([0.8, 0.7, 0.6],
                               [-1.5, 0.5, -1.5],
                               &mut meshes,
                               &mut materials));
    commands.spawn_bundle(light());
}
