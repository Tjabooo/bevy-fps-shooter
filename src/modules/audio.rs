use bevy::prelude::*;
use bevy::audio::prelude::AudioSink;
use crate::structs::{
    AudioController,
    GunController,
    Ambience
};

pub fn load_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ambience = asset_server.load("de_dust2_ambience.ogg");
    let gunshot = asset_server.load("ak-47.ogg");

    commands.insert_resource(AudioController {
        ambience_handle: ambience,
        gunshot_handle: gunshot
    });
}

pub fn audio_queues(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gun_query: Query<&GunController>,
    ambience_query: Query<Entity, With<Ambience>>,
    audio_controller: Res<AudioController>
) {
    for gun_controller in gun_query.iter() {
        if ambience_query.iter().next().is_none() {
            // de_dust2 ambience
            commands.spawn(
                AudioBundle {
                    source: audio_controller.ambience_handle.clone(),
                    settings: PlaybackSettings::ONCE
                }
            ).insert(Ambience);
        }
        
        if gun_controller.shooting && (gun_controller.just_pressed || gun_controller.bullet_delay.finished()) {
            // gunshot
            commands.spawn(
                AudioBundle {
                    source: audio_controller.gunshot_handle.clone(),
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