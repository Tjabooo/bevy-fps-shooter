use bevy::prelude::*;
use bevy::audio::prelude::AudioSink;
use crate::structs::{
    GunController,
    Ambience
};

pub fn audio_queues(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gun_query: Query<&GunController>,
    ambience_query: Query<Entity, With<Ambience>>
) {
    for gun_controller in gun_query.iter() {
        let ambience = asset_server.load("de_dust2_ambience.ogg");
        let bullet_sound = asset_server.load("ak-47.ogg");
        if ambience_query.iter().next().is_none() {
            // de_dust2 ambience
            commands.spawn(
                AudioBundle {
                    source: ambience.clone(),
                    settings: PlaybackSettings::ONCE
                }
            ).insert(Ambience);
        }
        
        if gun_controller.shooting && (gun_controller.just_pressed || gun_controller.bullet_delay.finished()) {
            // bullet fire
            commands.spawn(
                AudioBundle {
                    source: bullet_sound.clone(),
                    settings: PlaybackSettings::DESPAWN
                }
            );
        }
    }
}

pub fn audio_control(
    ambience_query: Query<&AudioSink, With<Ambience>>,
) {
    if let Ok(ambience_sink) = ambience_query.get_single() {
        ambience_sink.set_volume(0.3);
    }
}