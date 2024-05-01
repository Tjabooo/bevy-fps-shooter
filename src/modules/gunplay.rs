use crate::structs::{
    EnemyController,
    GunController,
    PlayerController,
    CameraController
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn update(
    mut commands: Commands,
    player_query: Query<(Entity, &Children), With<PlayerController>>,
    camera_query: Query<(&GlobalTransform, &Children), (With<CameraController>, Without<PlayerController>)>,
    mut gun_query: Query<&mut GunController>,
    mut enemy_query: Query<&mut EnemyController>,
    rapier_context: Res<RapierContext>,
) {
    let (player_entity, player_children) = player_query.single();
    for child in player_children.iter() {
        if let Ok((camera_transform, camera_children)) = camera_query.get(*child) {
            for child in camera_children.iter() {
                if let Ok(mut gun_controller) = gun_query.get_mut(*child) {
                    if gun_controller.shooting && (gun_controller.just_pressed || gun_controller.bullet_delay.finished()) {
                        gun_controller.bullet_delay.reset();
                        gun_controller.just_pressed = false;
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
                            flags: QueryFilterFlags::EXCLUDE_SENSORS | QueryFilterFlags::ONLY_FIXED,
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
                            //println!("Entity - {:?}", entity);
                            //println!("Enemy entity - {:?}", enemy_query.get_mut(entity));
                            if let Ok(mut enemy_controller) = enemy_query.get_mut(entity) {
                                if enemy_controller.health <= 0 {
                                    commands.entity(entity).despawn();
                                } else {
                                    enemy_controller.health -= 1;
                                }
                                //println!("{:?}", enemy_controller.health);
                            }
                        }
                    }
                }
            }
        }
    }
}