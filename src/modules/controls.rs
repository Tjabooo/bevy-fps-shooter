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

//#[derive(Resource)]
//pub struct Positions(Vec<Vec3>);
//
//impl Default for Positions {
//    fn default() -> Self {
//        Positions(Vec::new())
//    }
//}

// Handles player movement
pub fn update(
    key_event: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut PlayerController), Without<Camera3d>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut player_collider_query: Query<&mut Collider, With<PlayerController>>,
    mut gun_query: Query<&mut GunController>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    //mut positions: ResMut<Positions>
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
                      
                        horizontal_velocity = horizontal_velocity.normalize_or_zero();
                        
                        player.velocity.x = horizontal_velocity.x * speed * friction * crouch_modifier * delta_time;
                        player.velocity.y *= friction;
                        player.velocity.z = horizontal_velocity.z * speed * friction * crouch_modifier * delta_time;
                        
                        transform.translation += player.velocity;
    
                        // short 'script' to make it easier for me to create the levels
                        //if key_event.just_pressed(KeyCode::KeyE) {
                        //    // Round the translation to 4 decimal places
                        //    let rounded_translation = Vec3::new(
                        //        (transform.translation.x * 10000.0).round() / 10000.0,
                        //        ((transform.translation.y + 0.7) * 10000.0).round() / 10000.0,
                        //        (transform.translation.z * 10000.0).round() / 10000.0,                        
                        //    );                        
                        //    positions.0.push(rounded_translation);
                        //    println!("Positions: {:?}", positions.0);
                        //}
                    };
                }
            }
        }
    }
}