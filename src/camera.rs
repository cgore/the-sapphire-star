use bevy::prelude::*;

pub fn camera() -> bevy::prelude::OrthographicCameraBundle {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.0;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    camera
}
