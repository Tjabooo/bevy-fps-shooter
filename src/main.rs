mod modules;
mod rendering;

use bevy::{
    prelude::*,
    diagnostic::{
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin
    },
    window::{
        WindowTheme,
        PresentMode,
        WindowMode,
        CursorGrabMode,
        Cursor
    },
};
use modules::{
    game, 
    movement,
    player
};
use rendering::{
    lighting,
    entities
};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Test".into(),
                    resolution: (800., 600.).into(),
                    //mode: WindowMode::BorderlessFullscreen,
                    //resolution: (1920., 1080.).into(),
                    present_mode: PresentMode::AutoNoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    cursor: Cursor { 
                        icon: (CursorIcon::Arrow),
                        visible: (false),
                        grab_mode: (CursorGrabMode::Locked),
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
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            //RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, game::setup)
        .add_systems(Update, game::update)
        .add_systems(Startup, entities::setup)
        .add_systems(Startup, lighting::setup)
        .add_systems(Update, movement::update)
        .add_systems(Startup, player::setup)
        .add_systems(Update, player::mouse_callback)
        //.add_systems(Update, physics::gravity)
        .run();
}