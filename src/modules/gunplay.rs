use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::GameState;
use crate::structs::{
    TargetController,
    GunController,
    PlayerController,
    CameraController,
    StartButton,
    BulletTracer
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
    mut meshes: ResMut<Assets<Mesh>>
) {
    if let Ok((player_entity, player_children)) = player_query.get_single() {
        for child in player_children.iter() {
            if let Ok((camera_transform, camera_children)) = camera_query.get(*child) {
                for child in camera_children.iter() {
                    if let Ok(mut gun_controller) = gun_query.get_mut(*child) {
                        if let Ok(mut gun_transform) = gun_transform_query.get_single_mut() {
                            // fire function
                            if mouse_event.just_pressed(MouseButton::Left) {
                                gun_controller.shooting = true;
                                gun_controller.just_pressed = true;
                            } else if mouse_event.just_released(MouseButton::Left) {
                                gun_controller.shooting = false;
                            }
    
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
                                    let bullet = rapier_context.cast_ray_and_get_normal(
                                        bullet_ray.origin,
                                        *bullet_ray.direction,
                                        1000.0,
                                        true,
                                        QueryFilter::new().exclude_collider(player_entity)
                                    );
                                    if let Some((entity, intersection)) = bullet {
                                        // bullet tracer
                                        commands.spawn((
                                            PbrBundle {
                                                transform: Transform::from_translation(Vec3::new(
                                                    gun_transform.translation.x,
                                                    gun_transform.translation.y,
                                                    gun_transform.translation.z,                                                    
                                                )),
                                                mesh: meshes.add(Mesh::from(Cuboid { half_size: Vec3::splat(0.1) })),
                                                material: materials.add(StandardMaterial {
                                                    emissive: Color::rgb_linear(100., 100., 50.,),
                                                    ..Default::default()
                                                }),
                                                ..Default::default()
                                            },
                                            BulletTracer {
                                                direction: Vec3::ZERO,
                                                start_position: Vec3::new(
                                                    gun_transform.translation.x,
                                                    gun_transform.translation.y,
                                                    gun_transform.translation.z
                                                ),
                                                end_position: intersection.point,
                                                life_time: 0.3
                                            }
                                        ));
                                    }
                                    if let Some((entity, _toi)) = rapier_context.cast_ray(
                                        bullet_ray.origin,
                                        *bullet_ray.direction,
                                        1000.0,
                                        true,
                                        filter,
                                    ) {                                        
                                        info!("{:?} - {:?}", start_query.get(entity), entity);
                                        if let Ok(mut enemy_controller) = enemy_query.get_mut(entity) {
                                            enemy_controller.health -= 1;
                                            if enemy_controller.health <= 0 {
                                                commands.entity(entity).despawn();
                                            }
                                            //println!("{:?}", enemy_controller.health);
                                        } else if start_query.get(entity).is_ok() {
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

pub fn handle_tracers(
    mut commands: Commands,
    mut tracer_query: Query<(&mut BulletTracer, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut tracer, mut transform, entity) in tracer_query.iter_mut() {
        tracer.life_time -= time.delta_seconds();

        transform.translation = (tracer.start_position + tracer.end_position) / 2.;
        transform.scale.z = Vec3::distance(tracer.start_position, tracer.end_position);
        transform.scale.y = 0.003;
        transform.scale.x = 0.003;
        transform.look_at(tracer.end_position, Vec3::Y);

        if tracer.direction == Vec3::new(0., 0., 0.) {
            tracer.direction = Vec3::new(
                transform.forward().x,
                transform.forward().y,
                transform.forward().z,                
            );
        }
        tracer.start_position = move_tracer(
            tracer.start_position,
            tracer.end_position,
            time.delta_seconds() * 50.,
            time.delta_seconds()
        );
        if tracer.start_position == tracer.end_position {
            commands.entity(entity).despawn();
        }
    }
}

fn move_tracer(
    start_position: Vec3,
    end_position: Vec3,
    max_dist_delta: f32,
    delta_time: f32
) -> Vec3 {
    let tracer_vec = start_position - end_position;
    let tracer_magnitude = Vec3::length(tracer_vec);
    if tracer_magnitude <= delta_time * 50.0 || tracer_magnitude == 0.0 {
        return start_position;
    }
    return start_position + tracer_vec / tracer_magnitude * max_dist_delta;
}