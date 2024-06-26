use bevy::{
    audio::prelude::AudioSink,
    prelude::*
};
use crate::structs::{
    AudioController,
    GunController,
    Ambience,
    GameEntity
};

// Loads audio assets
pub fn load_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ambience = Some(asset_server.load("de_dust2_ambience.ogg"));
    let gunshot = Some(asset_server.load("ak-47.ogg"));

    commands.insert_resource(AudioController {
        ambience_handle: ambience,
        gunshot_handle: gunshot
    });
}

// Plays audio on condition
pub fn audio_playback(
    mut commands: Commands,
    gun_query: Query<&GunController>,
    ambience_query: Query<Entity, With<Ambience>>,
    audio_controller: Res<AudioController>
) {
    for gun_controller in gun_query.iter() {
        if let Some(ambience_handle) = &audio_controller.ambience_handle {
            if ambience_query.iter().next().is_none() {
                // plays ambience if it's not already playing
                commands.spawn((
                    AudioBundle {
                        source: ambience_handle.clone(),
                        settings: PlaybackSettings::ONCE
                    },
                    Ambience,
                    GameEntity,                    
                ));
            }
        }

        if let Some(gunshot_handle) = &audio_controller.gunshot_handle {
            // play gunshot every time i shoot
            if gun_controller.play_audio {
                commands.spawn((
                    AudioBundle {
                        source: gunshot_handle.clone(),
                        settings: PlaybackSettings::REMOVE
                    },
                    GameEntity,
                ));
            }
        }
    }
}

// Sets ambience volume to 0.3
pub fn audio_control(
    ambience_query: Query<&AudioSink, With<Ambience>>
) {
    if let Ok(ambience_sink) = ambience_query.get_single() {
        ambience_sink.set_volume(0.3);
    }
}