use bevy::prelude::*;
use crate::CollisionEvent;

pub fn display_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }
}