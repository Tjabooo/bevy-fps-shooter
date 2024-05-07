use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;
use crate::{
    GameState,
    LevelState
};
use crate::structs::{
    PlayerController,
    CameraController,
    FpsText,
    TargetText,
    GameEntity,
    EntityHandler,
    TargetController,
    LevelController,
};
use bevy::{
    prelude::*,
    input::mouse::MouseMotion,
    diagnostic::{
        DiagnosticsStore,
        FrameTimeDiagnosticsPlugin
    }, 
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Query<&mut Window>,
    target_controller_query: Query<&TargetController>
) {
    let mut window = window.get_single_mut().unwrap();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMonoNLNerdFont-Regular.ttf"),
                    font_size: 30.0,
                    ..Default::default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 30.0,
                    color: Color::GOLD,
                    ..Default::default()
                }
            } else {
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMonoNLNerdFont-Regular.ttf"),
                    font_size: 30.0,
                    ..Default::default()
                }
            }),
        ]),
        FpsText,
        GameEntity
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "TARGETS LEFT: ",
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMonoNLNerdFont-Regular.ttf"),
                    font_size: 30.0,
                    ..Default::default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 30.0,
                    color: Color::GOLD,
                    ..Default::default()
                }
            } else {
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMonoNLNerdFont-Regular.ttf"),
                    font_size: 30.0,
                    ..Default::default()
                }
            }),
        ]).with_style(Style {
            left: Val::Percent(44.5),
            ..Default::default()
        }),
        TargetText::default(),
        GameEntity
    ));
}

pub fn spawn_targets(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    entity_handler: Res<EntityHandler>,
    levels: Res<LevelController>
) {
    let material = materials.add(StandardMaterial { base_color_texture: entity_handler.target_texture_handle.clone(), ..Default::default() }); 
    let mesh = meshes.add(Sphere { radius: 0.1 });

    println!("{:?}", levels.level_1_pos);

    for target_position in levels.level_1_pos.iter() {
        commands
            .spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    transform: Transform::from_translation(*target_position),
                    ..Default::default()
                },
                AsyncCollider { ..Default::default() },
                RigidBody::Fixed,
                TargetController {
                    health: 1
                },
                GameEntity
            ));      
    }
}

pub fn mouse_callback(
    mut player_query: Query<&mut Transform, With<PlayerController>>,
    mut camera_query: Query<(&mut CameraController, &mut Transform),
                            (With<Camera>, Without<PlayerController>)>,
    mut mouse_motion_events: EventReader<MouseMotion>
) {
    for event in mouse_motion_events.read() {
        for (mut camera, mut camera_transform) in camera_query.iter_mut() {
            for mut player_transform in player_query.iter_mut() {
                const MAX_VERTICAL_ANGLE: f32 = std::f32::consts::FRAC_PI_2 - 0.02;
                
                camera.pitch += -event.delta.y * camera.sensitivity;
                camera.yaw += -event.delta.x * camera.sensitivity;
                
                camera.pitch = camera.pitch.clamp(-MAX_VERTICAL_ANGLE, MAX_VERTICAL_ANGLE);
    
                camera_transform.rotation = Quat::from_axis_angle(Vec3::X, camera.pitch);
                player_transform.rotation = Quat::from_axis_angle(Vec3::Y, camera.yaw);
            }
        }
    }
}

pub fn change_cursor_state(
    mut window: Query<&mut Window>
) {
    let mut window = window.get_single_mut().unwrap();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

pub fn diagnostics(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, Without<TargetText>>,
    mut target_text_query: Query<&mut Text, With<TargetText>>,
    target_controller_query: Query<&TargetController>
) {
    let mut target_text = target_text_query.get_single_mut().unwrap();
    let targets_left = target_controller_query.iter().count();

    target_text.sections[1].value = format!("{targets_left}");

    for mut fps_text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                fps_text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}

pub fn in_main_menu_state(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::MainMenu
}

pub fn in_pause_menu_state(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::PauseMenu
}

pub fn in_playing_state(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Playing
}

pub fn in_level_1_state(level_state: Res<State<LevelState>>) -> bool {
    level_state.get() == &LevelState::Level1
}