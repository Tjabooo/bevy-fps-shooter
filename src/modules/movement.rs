use bevy::prelude::*;
use crate::modules::player::Player;

pub fn update(
    key_event: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        let delta_time = time.delta_seconds();
        let jump_height = player.jump_height;
        let friction = 0.5;

        let forward = transform.forward();
        let backward = -forward;
        let right = transform.right();
        let left = -right;

        let mut horizontal_velocity = Vec3::ZERO;

        if key_event.pressed(KeyCode::W) {
            horizontal_velocity += forward;
        }
        if key_event.pressed(KeyCode::S) {
            horizontal_velocity += backward;
        }
        if key_event.pressed(KeyCode::A) {
            horizontal_velocity += left;
        }
        if key_event.pressed(KeyCode::D) {
            horizontal_velocity += right;
        }
        if key_event.just_pressed(KeyCode::Space) {
            player.velocity.y += jump_height;
        }
        if key_event.pressed(KeyCode::ShiftLeft) {
            player.speed = 2.5;
        } else {
            player.speed = 4.0;
        }
        
        player.velocity = player.velocity.normalize_or_zero();
        horizontal_velocity = horizontal_velocity.normalize_or_zero();

        player.velocity.x = horizontal_velocity.x * friction;
        player.velocity.z = horizontal_velocity.z * friction;

        transform.translation += player.velocity * player.speed * delta_time;
    }
}