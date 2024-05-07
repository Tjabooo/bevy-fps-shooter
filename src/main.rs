mod modules;
mod rendering;

use crate::structs::{
    PlayerController,
    MapController,
    GunController,
    AudioController,
    CubemapController,
    LevelController,
    EntityHandler,
    TargetController
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_scene_hook::HookPlugin;
use bevy_rapier3d::prelude::*;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{
        Cursor,
        CursorGrabMode,
        PresentMode, 
        WindowMode,
        WindowResolution,
        WindowTheme
    }
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
    MainMenu,
    PauseMenu,
    Playing,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default)]
pub enum LevelState {
    #[default]
    Level1,
    Level2
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
    .insert_state(GameState::MainMenu)
    .insert_state(LevelState::Level1)
    .insert_resource(Msaa::Sample8)
    .init_resource::<PlayerController>() 
    .init_resource::<TargetController>()
    .init_resource::<MapController>()
    .init_resource::<GunController>()  
    .init_resource::<AudioController>()
    .init_resource::<CubemapController>()
    .init_resource::<LevelController>()
    .init_resource::<EntityHandler>()
    // main menu
    .add_systems(OnEnter(GameState::MainMenu), menu::setup_main_menu)
    .add_systems(Update, menu::menu_interactions.run_if(game::in_main_menu_state))
    // pause menu
    .add_systems(OnEnter(GameState::PauseMenu), menu::setup_pause_menu)
    .add_systems(Update, menu::menu_interactions.run_if(game::in_pause_menu_state))
    // game
    .add_systems(OnTransition {
        from: GameState::MainMenu, 
        to: GameState::Playing
    }, (
        game::setup,
        entities::setup,
        lighting::setup,
    ))
    .add_systems(OnEnter(LevelState::Level1), game::spawn_targets)
    .add_systems(Update, (
        game::mouse_callback,
        game::diagnostics,
        entities::rotate_map,
        entities::rotate_gun,
        entities::load_cubemap,
        controls::update,
        gunplay::update,
        audio::audio_playback,
        audio::audio_control
    ).run_if(game::in_playing_state))
    // cleanup systems
    .add_systems(OnExit(GameState::MainMenu), entities::despawn_menu_entities)
    .add_systems(OnExit(GameState::PauseMenu), entities::despawn_menu_entities)
    .add_systems(OnTransition {
        from: GameState::PauseMenu,
        to: GameState::MainMenu
    }, entities::despawn_game_entities)
    // misc
    .add_systems(Startup, (entities::load_entities, audio::load_audio))
    .add_systems(OnTransition {
        from: GameState::PauseMenu,
        to: GameState::Playing
    }, game::change_cursor_state)
    .run();
}