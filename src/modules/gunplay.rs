use crate::rendering::entities::{EnemyController, GunController, PlayerController};
use crate::game::CameraController;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup(
    mut commands: Commands 
) {

}

pub fn update(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Children), With<PlayerController>>,
    mut camera_query: Query<(&GlobalTransform, &Children), (With<CameraController>, Without<PlayerController>)>,
    mut gun_query: Query<&GunController>,
    mut enemy_query: Query<&mut EnemyController>,
    rapier_context: Res<RapierContext>,
) {
    if let (player_entity, player_children) = player_query.single() {
        for child in player_children.iter() {
            if let Ok((camera_transform, camera_children)) = camera_query.get(*child) {
                if let Ok(gun_controller) = gun_query.get(*child) {
                    if gun_controller.shoot {
                        let bullet_ray = Ray3d {
                            origin: camera_transform.translation(),
                            direction: Direction3d::new(Vec3::new(
                                camera_transform.forward().x,
                                camera_transform.forward().y,
                                camera_transform.forward().z,
                            ))
                            .unwrap(),
                        };

                        let filter = QueryFilter {
                            flags: QueryFilterFlags::EXCLUDE_SENSORS | QueryFilterFlags::ONLY_DYNAMIC,
                            exclude_collider: Some(player_entity),
                            groups: None,
                            ..Default::default()
                        };

                        if let Some((entity, _toi)) = rapier_context.cast_ray(
                            bullet_ray.origin,
                            *bullet_ray.direction,
                            1000.0,
                            true,
                            filter,
                        ) {
                            println!("{:?}", entity);
        
                            if let Ok(mut enemy_controller) = enemy_query.get_mut(entity) {
                                enemy_controller.health -= 1;
                            }
                        }
                    }
                }
            }
        }
    }
}