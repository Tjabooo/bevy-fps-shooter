mod modules;
mod rendering;

use bevy_scene_hook::HookPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
    diagnostic::{
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin
    } 
};
use modules::{
    game, 
    controls,
    animations,
    gunplay
};
use rendering::{
    lighting,
    entities
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Valorant".into(),
                    //resolution: (800., 600.).into(),
                    mode: WindowMode::BorderlessFullscreen,
                    resolution: WindowResolution::new(1920., 1080.),
                    present_mode: PresentMode::AutoNoVsync,
                    //fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    cursor: Cursor { 
                        icon: default(),
                        visible: (false),
                        grab_mode: (CursorGrabMode::Locked),
                        //visible: (true),
                        //grab_mode: (CursorGrabMode::None),
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
            //WorldInspectorPlugin::new()
        ))
        .insert_resource(Msaa::Sample8)
        .add_systems(Startup, game::setup)
        .add_systems(Update, game::update)
        .add_systems(Startup, entities::setup)
        .add_systems(Update, entities::rotate_map)
        .add_systems(Update, entities::rotate_gun)
        .add_systems(Update, entities::load_cubemap)
        .add_systems(Startup, lighting::setup)
        .add_systems(Update, controls::update)
        .add_systems(Update, gunplay::update)
        .add_systems(Update, game::update)
        .add_systems(Update, game::diagnostics)
        .run();
}