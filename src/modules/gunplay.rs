use crate::rendering::entities::{EnemyController, GunController};
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
    mut camera_query: Query<&GlobalTransform, With<CameraController>>,
    rapier_context: Res<RapierContext>,
    mut enemy_query: Query<&mut EnemyController>
) {
    for gun_controller in gun_query.iter_mut() {
        for camera_transform in camera_query.iter_mut() {
            if gun_controller.shoot {
                let bullet_ray = Ray3d {
                    origin: camera_transform.translation(),
                    direction: 
                        Direction3d::new(
                            Vec3::new(
                                camera_transform.forward().x,
                                camera_transform.forward().y,
                                camera_transform.forward().z
                            )
                        ).unwrap()
                };
                if let Some((entity, _toi)) = rapier_context.cast_ray(
                    bullet_ray.origin,
                    *bullet_ray.direction,
                    100.0,
                    true,
                    QueryFilter::only_dynamic()
                ) {
                    println!("{:?}", entity);

                    if let Ok(mut enemy_controller) = enemy_query.get_mut(entity) {
                        enemy_controller.health -= 1;
                        println!("{:?}", enemy_controller.health);
                    }
                }
            }
        }
    }
}