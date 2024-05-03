mod modules;
mod rendering;

use bevy_scene_hook::HookPlugin;
use bevy_rapier3d::prelude::*;
use bevy::{
    prelude::*,
    window::{
        Cursor,
        CursorGrabMode,
        PresentMode, 
        WindowMode,
        WindowResolution,
        WindowTheme
    },
    diagnostic::FrameTimeDiagnosticsPlugin
};
use modules::{
    game, 
    controls,
    audio,
    gunplay,
    structs,
    menu
};
use rendering::{
    lighting,
    entities
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Playing,
}

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "VALORANT 2.0".into(),
                //resolution: (800., 600.).into(),
                mode: WindowMode::BorderlessFullscreen,
                resolution: WindowResolution::new(1920., 1080.),
                present_mode: PresentMode::AutoNoVsync,
                window_theme: Some(WindowTheme::Dark),
                cursor: Cursor { 
                    icon: default(),
                    //visible: (false),
                    //grab_mode: (CursorGrabMode::Locked),
                    visible: (true),
                    grab_mode: (CursorGrabMode::None),
                    hit_test: (true)
                },
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                visible: true,
                ..default()
            }),
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
            ..default()
        }),
        //LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        //RapierDebugRenderPlugin::default(),
        HookPlugin,
        //WorldInspectorPlugin::new(),
    ))
    .insert_state(GameState::Splash)
    .insert_resource(Msaa::Sample8)
    .add_systems(Startup, (
        game::setup,
        entities::setup,
        entities::spawn_enemies,
        lighting::setup,
        audio::load_audio
    ).run_if(game::in_splash_state))
    // main menu
    .add_systems(Startup, menu::setup.run_if(game::in_splash_state))
    .add_systems(Update, menu::menu_interaction.run_if(game::in_splash_state))
    // in-game
    //.add_systems(Update, entities::load_cubemap)
    .add_systems(Update, (
        game::update,
        game::diagnostics,
        entities::rotate_map,
        entities::rotate_gun,
        entities::load_cubemap,
        //controls::update,
        gunplay::update,
        audio::audio_queues,
        audio::audio_control
    ))
    //.run_if(game::in_playing_state))
    .add_systems(Update, controls::update.run_if(game::in_playing_state))
    .run();
}