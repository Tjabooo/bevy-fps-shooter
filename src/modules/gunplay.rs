use crate::rendering::entities::GunController;
use crate::game::CameraController;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup(
    mut commands: Commands 
) {

}

pub fn update(
    mut commands: Commands,
    mut gun_query: Query<&GunController>,
    mut camera_query: Query<&Transform, With<CameraController>>
) {
    for gun_controller in gun_query.iter_mut() {
        for camera_transform in camera_query.iter_mut() {
            if gun_controller.shoot {
                let camera_rotations = camera_transform.rotation.to_euler(EulerRot::XYZ);
                println!("{:?}", camera_rotations);
                let bullet_ray = Ray3d {
                    origin: camera_transform.translation,
                    direction: 
                        Direction3d::new(
                            Vec3::new(
                                camera_rotations.0,
                                camera_rotations.1,
                                camera_rotations.2
                            )).unwrap()
                };
            }
        }
    }
}