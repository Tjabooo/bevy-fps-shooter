use crate::GameState;
use crate::game;
use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::*,
    geometry::Collider,
    plugin::RapierContext
};
use crate::structs::{
    PlayerController, 
    GunController
};

pub fn update(
    key_event: Res<ButtonInput<KeyCode>>,
    mouse_event: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<(&mut Transform, &mut PlayerController), Without<Camera3d>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut player_collider_query: Query<&mut Collider, With<PlayerController>>,
    mut gun_query: Query<&mut GunController>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        for mut camera in camera_query.iter_mut() {
            for mut player_collider in player_collider_query.iter_mut() {
                for mut gun_controller in gun_query.iter_mut() { 
                    if let Some(bullet_delay) = &mut gun_controller.bullet_delay {
                        let delta_time = time.delta_seconds();
                        let jump_height = player.jump_height;
                        let friction = 0.9;
                        let mut speed = player.speed;
                        let mut crouch_modifier = player.crouch_modifier;
                        
                        bullet_delay.tick(time.delta());
                        
                        let grounded_ray = Ray3d {
                            origin: transform.translation,
                            direction: Direction3d::new(-Vec3::Y).unwrap(),
                        };
                        
                        player.is_grounded = rapier_context.cast_ray(
                            grounded_ray.origin,
                            *grounded_ray.direction,
                            0.4,
                            true,
                            QueryFilter::only_fixed(),
                        ).is_some();
                        
                        let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z).normalize_or_zero();
                        let backward = -forward;
                        let right = Vec3::new(transform.right().x, 0.0, transform.right().z).normalize_or_zero();
                        let left = -right;
        
                        let mut horizontal_velocity = Vec3::ZERO;
                        
                        if player.is_crouched {
                            crouch_modifier = 0.4;
                        }
                        if key_event.pressed(KeyCode::KeyW) {
                            horizontal_velocity += forward;
                        }
                        if key_event.pressed(KeyCode::KeyS) {
                            horizontal_velocity += backward;
                        }
                        if key_event.pressed(KeyCode::KeyA) {
                            horizontal_velocity += left;
                        }
                        if key_event.pressed(KeyCode::KeyD) {
                            horizontal_velocity += right;
                        }
                        if key_event.just_pressed(KeyCode::Space) && player.is_grounded {
                            player.velocity.y = jump_height;
                        }        
                        if key_event.pressed(KeyCode::ShiftLeft) {
                            speed /= 1.7;
                        }
                        if key_event.pressed(KeyCode::ControlLeft) {
                            camera.translation = Vec3::new(0.0, 0.300, 0.0);
                            if !player.is_crouched {
                                //transform.translation.y -= 0.1345; // cuboid bug fix
                                if let Some(mut capsule) = player_collider.as_capsule_mut() {
                                    capsule.set_segment(Vec3::ZERO, Vec3::new(0.0, 0.130, 0.0));
                                }
                                player.is_crouched = true;
                              }
                        } else {
                            camera.translation = Vec3::new(0.0, 0.650, 0.0);
                            if player.is_crouched {
                                //transform.translation.y += 0.1345; // cuboid bug fix
                                if let Some(mut capsule) = player_collider.as_capsule_mut() {
                                    capsule.set_segment(Vec3::ZERO, Vec3::new(0.0, 0.450, 0.0));
                                }
                                player.is_crouched = false; 
                              }
                        }
                        if mouse_event.just_pressed(MouseButton::Left) {
                            gun_controller.shooting = true;
                            gun_controller.just_pressed = true;
                        } else if mouse_event.just_released(MouseButton::Left) {
                            gun_controller.shooting = false;
                        }
                        if key_event.just_pressed(KeyCode::Escape) {
                            if *state.get() == GameState::Playing {
                                next_state.set(GameState::PauseMenu);
                            }
                        }
                        
                        horizontal_velocity = horizontal_velocity.normalize_or_zero();
                        
                        player.velocity.x = horizontal_velocity.x * speed * friction * crouch_modifier * delta_time;
                        player.velocity.y *= friction;
                        player.velocity.z = horizontal_velocity.z * speed * friction * crouch_modifier * delta_time;
                        
                        transform.translation += player.velocity;
    
                        //info!("{:?}", transform.translation);
                    };
                }
            }
        }
    }
}