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
    TimeText,
    TimeController,
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

use super::structs::StartButton;

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

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "TIME: ",
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
            left: Val::Percent(45.5),
            top: Val::Percent(2.5),
            ..Default::default()
        }),
        TimeText,
        GameEntity
    ));
}

pub fn update(
    key_event: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_level: ResMut<NextState<LevelState>>,
    mut time_controller: ResMut<TimeController>
) {
    if key_event.just_pressed(KeyCode::Escape) {
        if *current_state.get() == GameState::Playing || *current_state.get() == GameState::Start {
            next_state.set(GameState::PauseMenu);
        }
    }

    if time_controller.is_finished() {
        next_level.set(LevelState::Failed);
        time_controller.time_left = None;
    }
}

pub fn initiate_level(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    entity_handler: Res<EntityHandler>,
    levels: Res<LevelController>,
    current_level: Res<State<LevelState>>,
    player_controller: Res<PlayerController>,
    mut player_query: Query<&mut Transform, With<PlayerController>>,
    mut time_controller: ResMut<TimeController>
) {
    let material = materials.add(StandardMaterial { base_color_texture: entity_handler.target_texture_handle.clone(), ..Default::default() }); 
    let mesh = meshes.add(Sphere { radius: 0.1 });
    
    let TimeController { 
        level_1_time, 
        level_2_time, 
        level_3_time, 
        .. 
    } = TimeController::default();

    for mut player_transform in player_query.iter_mut() {
        *player_transform = Transform::from_translation(
            Vec3::new(
                player_controller.spawn_point.x,
                player_controller.spawn_point.y,
                player_controller.spawn_point.z
            )
        );
    }

    commands
        .spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(
                    Vec3::new(
                        player_controller.spawn_point.x,
                        player_controller.spawn_point.y + 0.2,
                        player_controller.spawn_point.z - 4.9
                    )
                ),
                ..Default::default()
            },
            AsyncCollider { ..Default::default() },
            RigidBody::Fixed,
            StartButton,
            GameEntity
        ));

    println!("Got here 4!");

    match current_level.get() {
        LevelState::Failed => {
            println!(":3");
        }
        LevelState::Level1 => {
            time_controller.set_timer(level_1_time);
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
        LevelState::Level2 => {
            time_controller.set_timer(level_2_time);
            for target_position in levels.level_2_pos.iter() {
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
        LevelState::Level3 => {
            time_controller.set_timer(level_3_time);
            for target_position in levels.level_3_pos.iter() {
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
    }

}

pub fn update_level_timer(
    mut time_controller: ResMut<TimeController>,
    time: Res<Time>
) {
    time_controller.start_timer(time.delta());    
}

pub fn change_level_state(
    target_query: Query<Entity, With<TargetController>>,
    current_level: Res<State<LevelState>>,
    current_state: Res<State<GameState>>,
    time_controller: Res<TimeController>,
    mut next_level: ResMut<NextState<LevelState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Transform, &PlayerController)>,
) {  
    println!("{:?}", current_state.get());
    println!("{:?}", current_level.get());
    println!("{:?}", time_controller);
    if target_query.iter().count() <= 0 {
        match current_level.get() {
            LevelState::Failed => {
                next_level.set(LevelState::Level1);
                next_state.set(GameState::Start);
            }
            LevelState::Level1 => {
                next_level.set(LevelState::Level2);
                next_state.set(GameState::Start);
            }
            LevelState::Level2 => {
                next_level.set(LevelState::Level3);
                next_state.set(GameState::Start);
            }
            LevelState::Level3 => {
                
            }
        }
    }
    //println!("{:?}", target_query.iter().count());
    //println!("{:?}", current_level.get());
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
    mut fps_text_query: Query<&mut Text, (With<FpsText>, Without<TargetText>, Without<TimeText>)>,
    mut target_text_query: Query<&mut Text, (With<TargetText>, Without<TimeText>)>,
    target_controller_query: Query<&TargetController>,
    mut time_left_query: Query<&mut Text, (With<TimeText>, Without<TargetText>)>,
    time_controller: Res<TimeController>
) {
    let mut target_text = target_text_query.get_single_mut().unwrap();
    let targets_left = target_controller_query.iter().count();
    let mut time_text = time_left_query.get_single_mut().unwrap();

    target_text.sections[1].value = format!("{targets_left}");
    time_text.sections[1].value = time_controller.get_time_left();

    for mut fps_text in &mut fps_text_query {
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

pub fn in_start_state(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Start
}

pub fn in_playing_state(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Playing
}

pub fn in_failed_state(level_state: Res<State<LevelState>>) -> bool {
    level_state.get() == &LevelState::Failed
}

pub fn in_level_1_state(level_state: Res<State<LevelState>>) -> bool {
    level_state.get() == &LevelState::Level1
}