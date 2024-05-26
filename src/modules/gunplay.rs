use bevy_rapier3d::prelude::*;
use crate::{
    GameState,
    structs::{
        TargetController,
        GunController,
        PlayerController,
        CameraController,
        StartButton,
        PlayerEntity,
        GameEntity,
    }
};
use bevy::{
    prelude::*,
    render::view::NoFrustumCulling
};

pub fn update(
    mut commands: Commands,
    mouse_event: Res<ButtonInput<MouseButton>>,
    player_query: Query<(Entity, &Children), With<PlayerController>>,
    camera_query: Query<(&GlobalTransform, &Children), (With<CameraController>, Without<PlayerController>)>,    
    mut gun_query: Query<&mut GunController>,
    mut gun_transform_query: Query<&mut Transform, With<GunController>>,
    mut enemy_query: Query<&mut TargetController>,
    mut start_query: Query<Entity, (With<StartButton>, Without<PlayerController>)>,
    rapier_context: Res<RapierContext>,
    mut next_state: ResMut<NextState<GameState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    player_entity: Res<PlayerEntity>,
    player_controller: Res<PlayerController>
) {
    if let Ok((player_entity, player_children)) = player_query.get_single() {
        for child in player_children.iter() {
            if let Ok((camera_transform, camera_children)) = camera_query.get(*child) {
                for child in camera_children.iter() {
                    if let Ok(mut gun_controller) = gun_query.get_mut(*child) {
                        if let Ok(mut gun_transform) = gun_transform_query.get_single_mut() {
                            // fires gun when pressing LMB
                            if mouse_event.just_pressed(MouseButton::Left) {
                                gun_controller.shooting = true;
                                gun_controller.just_pressed = true;
                            } else if mouse_event.just_released(MouseButton::Left) {
                                gun_controller.shooting = false;
                            }
    
                            let shooting = gun_controller.shooting;
                            let just_pressed = gun_controller.just_pressed;                        
                            
                            // shoots gun if conditions are met
                            if let Some(bullet_delay) = &mut gun_controller.bullet_delay {
                                if shooting && (just_pressed || bullet_delay.finished()) {
                                    bullet_delay.reset();
                                    gun_controller.just_pressed = false;

                                    // bullet raycast
                                    let bullet_ray = Ray3d {
                                        origin: camera_transform.translation(),
                                        direction: Direction3d::new(Vec3::new(
                                            camera_transform.forward().x,
                                            camera_transform.forward().y,
                                            camera_transform.forward().z,
                                        ))
                                        .unwrap(),
                                    };
                                    
                                    // raycast query filter
                                    let filter = QueryFilter {
                                        flags: QueryFilterFlags::EXCLUDE_SENSORS | QueryFilterFlags::ONLY_FIXED,
                                        exclude_collider: Some(player_entity),
                                        groups: None,
                                        ..Default::default()
                                    };

                                    // bullet raycast
                                    if let Some((entity, toi)) = rapier_context.cast_ray(
                                        bullet_ray.origin,
                                        *bullet_ray.direction,
                                        1000.0,
                                        true,
                                        filter,
                                    ) {                                        
                                        info!("{:?} - {:?}", start_query.get(entity), entity);

                                        // despawn target if raycast entity id matches target entity id
                                        if let Ok(mut enemy_controller) = enemy_query.get_mut(entity) {
                                            enemy_controller.health -= 1;
                                            if enemy_controller.health <= 0 {
                                                commands.entity(entity).despawn();
                                            }
                                        }
                                        // starts game if start button gets shot
                                        else if start_query.get(entity).is_ok() {
                                            commands.entity(entity).despawn();
                                            next_state.set(GameState::Playing);
                                        }
                                        //println!("Entity - {:?}", entity);
                                        //println!("Enemy entity - {:?}", enemy_query.get_mut(entity));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}