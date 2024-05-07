use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::structs::{
    TargetController,
    GunController,
    PlayerController,
    CameraController
};

pub fn update(
    mut commands: Commands,
    player_query: Query<(Entity, &Children), With<PlayerController>>,
    camera_query: Query<(&GlobalTransform, &Children), (With<CameraController>, Without<PlayerController>)>,
    mut gun_query: Query<&mut GunController>,
    mut enemy_query: Query<&mut TargetController>,
    rapier_context: Res<RapierContext>
) {
    if let Ok((player_entity, player_children)) = player_query.get_single() {
        for child in player_children.iter() {
            if let Ok((camera_transform, camera_children)) = camera_query.get(*child) {
                for child in camera_children.iter() {
                    if let Ok(mut gun_controller) = gun_query.get_mut(*child) {
                        let shooting = gun_controller.shooting;
                        let just_pressed = gun_controller.just_pressed;
                        if let Some(bullet_delay) = &mut gun_controller.bullet_delay {
                            if shooting && (just_pressed || bullet_delay.finished()) {
                                bullet_delay.reset();
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
                                    if let Ok(mut enemy_controller) = enemy_query.get_mut(entity) {
                                        enemy_controller.health -= 1;
                                        if enemy_controller.health <= 0 {
                                            commands.entity(entity).despawn();
                                        }
                                        //println!("{:?}", enemy_controller.health);
                                    }
                                    //println!("Entity - {:?}", entity);
                                    //println!("Enemy entity - {:?}", enemy_query.get_mut(entity));
                                }
                            }
                        };
                    }
                }
            }
        }
    }
}