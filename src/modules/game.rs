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
    LevelText,
    TimeController,
    GameEntity,
    EntityHandler,
    TargetController,
    LevelController,
    StartButton,
    LastState,
    MapImage
};
use bevy::{
    window::CursorGrabMode,
    prelude::*,
    input::mouse::MouseMotion,
    diagnostic::{
        DiagnosticsStore,
        FrameTimeDiagnosticsPlugin
    }, 
};

// Runs on startup and spawns overlay text
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Query<&mut Window>,
) {
    let mut window = window.get_single_mut().unwrap();
    let text_font = asset_server.load("fonts/JetBrainsMonoNLNerdFont-Regular.ttf");

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    // spawn fps overlay text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(
                TextStyle {
                    font: text_font.clone(),
                    font_size: 30.0,
                    ..Default::default()
                }
            )         
        ]).with_style(Style {
            left: Val::Percent(0.35),
            top: Val::Percent(0.2),
            ..Default::default()
        }),
        FpsText,
        GameEntity
    ));

    // spawn level/difficulty overlay text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(
                TextStyle {
                    font: text_font.clone(),
                    font_size: 30.0,
                    ..Default::default()
                }
            )         
        ]).with_style(Style {
            justify_self: JustifySelf::Center,
            ..Default::default()
        }),
        LevelText,
        GameEntity
    ));

    // spawn targets left overlay text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(
                TextStyle {
                    font: text_font.clone(),
                    font_size: 30.0,
                    ..Default::default()
                }
            )         
        ]).with_style(Style {
            justify_self: JustifySelf::Center,
            top: Val::Percent(2.5),
            ..Default::default()
        }),
        TargetText { targets_left: None },
        GameEntity
    ));

    // spawn time overlay text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(
                TextStyle {
                    font: text_font.clone(),
                    font_size: 30.0,
                    ..Default::default()
                }
            )         
        ]).with_style(Style {
            justify_self: JustifySelf::Center,
            top: Val::Percent(5.0),
            ..Default::default()
        }),
        TimeText,
        GameEntity
    ));
}

// Handles pause menu and the timer once finished
pub fn update(
    key_event: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut last_state: ResMut<LastState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_level: ResMut<NextState<LevelState>>,
    mut time_controller: ResMut<TimeController>
) {
    let state = *current_state.get();

    if key_event.just_pressed(KeyCode::Escape) {
        if state == GameState::Playing || state == GameState::Start || state == GameState::Won {
            last_state.state = Some(state);
            next_state.set(GameState::PauseMenu);
        }
    }

    if time_controller.is_finished() {
        next_level.set(LevelState::Failed);
        time_controller.time_left = None;
    }
}

// Runs every level change, handles and spawns targets/start button/map image
pub fn initiate_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    entity_handler: Res<EntityHandler>,
    levels: Res<LevelController>,
    current_level: Res<State<LevelState>>,
    player_controller: Res<PlayerController>,
    mut player_query: Query<&mut Transform, With<PlayerController>>,
    mut time_controller: ResMut<TimeController>,
    map_image_query: Query<Entity, With<MapImage>>
) {
    let ball_material = materials.add(
        StandardMaterial { 
            base_color_texture: entity_handler.target_texture_handle.clone(), 
            ..Default::default() 
        }
    ); 
    let ball_mesh = meshes.add(
        Sphere { 
            radius: 0.1 
        }
    );

    let mut map_image_handle: StandardMaterial = Color::SILVER.into();

    let TimeController { 
        level_1_time, 
        level_2_time, 
        level_3_time, 
        level_4_time,
        level_5_time,
        .. 
    } = TimeController::default();

    // move player entity to spawn point
    for mut player_transform in player_query.iter_mut() {
        *player_transform = Transform::from_translation(
            Vec3::new(
                player_controller.spawn_point.x,
                player_controller.spawn_point.y,
                player_controller.spawn_point.z
            )
        );
    }

    // spawn start button entity
    commands.spawn((
        PbrBundle {
            mesh: ball_mesh.clone(),
            material: ball_material.clone(),
            transform: Transform::from_translation(
                Vec3::new(
                    player_controller.spawn_point.x,
                    player_controller.spawn_point.y,
                    player_controller.spawn_point.z - 0.7
                )
            ),
            ..Default::default()
        },
        AsyncCollider { ..Default::default() },
        RigidBody::Fixed,
        StartButton,
        GameEntity
    ));

    match current_level.get() {
        LevelState::NoLevel => {}
        LevelState::Failed => {}
        LevelState::Level1 => {
            // set timer
            time_controller.set_timer(level_1_time);
            
            // spawn map image path
            map_image_handle = asset_server.load("levels/level1.png").into();

            // spawn targets
            for target_position in levels.level_1_pos.iter() {
                commands.spawn((
                    PbrBundle {
                        mesh: ball_mesh.clone(),
                        material: ball_material.clone(),
                        transform: Transform::from_translation(*target_position),
                        ..Default::default()
                    },
                    AsyncCollider { ..Default::default() },
                    RigidBody::Fixed,
                    TargetController { health: 1 },
                    GameEntity
                ));
            }
        }
        LevelState::Level2 => {
            // set timer
            time_controller.set_timer(level_2_time);

            // set map image asset
            map_image_handle = asset_server.load("levels/level2.png").into();

            // spawn targets
            for target_position in levels.level_2_pos.iter() {
                commands.spawn((
                    PbrBundle {
                        mesh: ball_mesh.clone(),
                        material: ball_material.clone(),
                        transform: Transform::from_translation(*target_position),
                        ..Default::default()
                    },
                    AsyncCollider { ..Default::default() },
                    RigidBody::Fixed,
                    TargetController { health: 1 },
                    GameEntity
                ));
            }
        }
        LevelState::Level3 => {
            // set timer
            time_controller.set_timer(level_3_time);

            // set map image 
            map_image_handle = asset_server.load("levels/level3.png").into();

            // spawn targets
            for target_position in levels.level_3_pos.iter() {
                commands.spawn((
                    PbrBundle {
                        mesh: ball_mesh.clone(),
                        material: ball_material.clone(),
                        transform: Transform::from_translation(*target_position),
                        ..Default::default()
                    },
                    AsyncCollider { ..Default::default() },
                    RigidBody::Fixed,
                    TargetController { health: 1 },
                    GameEntity
                ));
            }
        }
        LevelState::Level4 => {
            //set timer
            time_controller.set_timer(level_4_time);

            // set map image
            map_image_handle = asset_server.load("levels/level4.png").into();

            // spawn targets
            for target_position in levels.level_4_pos.iter() {
                commands.spawn((
                    PbrBundle {
                        mesh: ball_mesh.clone(),
                        material: ball_material.clone(),
                        transform: Transform::from_translation(*target_position),
                        ..Default::default()
                    },
                    AsyncCollider { ..Default::default() },
                    RigidBody::Fixed,
                    TargetController { health: 1 },
                    GameEntity
                ));
            }
        }
        LevelState::Level5 => {
            //set timer
            time_controller.set_timer(level_5_time);

            // set map image
            map_image_handle = asset_server.load("levels/level5.png").into();

            // spawn targets
            for target_position in levels.level_5_pos.iter() {
                commands.spawn((
                    PbrBundle {
                        mesh: ball_mesh.clone(),
                        material: ball_material.clone(),
                        transform: Transform::from_translation(*target_position),
                        ..Default::default()
                    },
                    AsyncCollider { ..Default::default() },
                    RigidBody::Fixed,
                    TargetController { health: 1 },
                    GameEntity
                ));
            }
        }
    }

    // despawn old map image
    if let Ok(map_image_entity) = map_image_query.get_single() {
        commands.entity(map_image_entity).despawn();
    }

    // spawn map image
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(2.5, 2.5)),
        material: materials.add(map_image_handle),
        transform: Transform {
            translation: Vec3::new(
                player_controller.spawn_point.x,
                player_controller.spawn_point.y + 0.75,
                player_controller.spawn_point.z - 2.0,
            ),
            rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            ..Default::default()
        },
        ..Default::default()
    }).insert((MapImage, GameEntity));
}

// This method updates the timer every frame
pub fn update_level_timer(
    mut time_controller: ResMut<TimeController>,
    time: Res<Time>
) {
    time_controller.run_timer(time.delta());
}

// This method changes level state on call
pub fn change_level_state(
    target_query: Query<Entity, With<TargetController>>,
    current_level: Res<State<LevelState>>,
    mut next_level: ResMut<NextState<LevelState>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    // check if all targets are destroyed
    if target_query.iter().count() <= 0 {
        match current_level.get() {
            LevelState::NoLevel => {}
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
                next_level.set(LevelState::Level4);
                next_state.set(GameState::Start);
            }
            LevelState::Level4 => {
                next_level.set(LevelState::Level5);
                next_state.set(GameState::Start);
            }
            LevelState::Level5 => {
                next_level.set(LevelState::NoLevel);
                next_state.set(GameState::Won);
            }
        }
    }
}

// This method handles mouse motion
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

// Changes cursor state in/out of menu
pub fn change_cursor_state(
    mut window: Query<&mut Window>
) {
    let mut window = window.get_single_mut().unwrap();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

// Handles the text overlay
pub fn diagnostics(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut Text, (With<FpsText>, Without<TargetText>, Without<TimeText>, Without<LevelText>)>,
    mut target_text_query: Query<&mut Text, (With<TargetText>, Without<TimeText>, Without<LevelText>)>,
    target_controller_query: Query<&TargetController>,
    mut time_left_query: Query<&mut Text, (With<TimeText>, Without<TargetText>, Without<LevelText>)>,
    time_controller: Res<TimeController>,
    mut level_text_query: Query<&mut Text, (With<LevelText>, Without<TargetText>, Without<TimeText>)>,
    current_level: Res<State<LevelState>>
) {
    let mut fps_text = fps_text_query.get_single_mut().unwrap();
    let Some(raw_fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) else { return };
    let Some(fps) = raw_fps.smoothed() else { return };

    let mut target_text = target_text_query.get_single_mut().unwrap();
    let targets_left = target_controller_query.iter().count();
    
    let mut time_text = time_left_query.get_single_mut().unwrap();

    let mut level_text = level_text_query.get_single_mut().unwrap();    
    let level_info = match current_level.get() {
        LevelState::NoLevel => ["NONE", "NONE"],
        LevelState::Level1 => ["1", "VERY EASY"],
        LevelState::Level2 => ["2", "EASY"],
        LevelState::Level3 => ["3", "MEDIUM"],
        LevelState::Level4 => ["4", "HARD"],
        LevelState::Level5 => ["5", "IMPOSSIBLE"],
        LevelState::Failed => ["FAILED", "FAILED"]
    };

    fps_text.sections[0].value = format!(
        "FPS: {}", fps.round()
    );
    level_text.sections[0].value = format!(
        "LEVEL {} - {}", level_info[0].to_string(), level_info[1].to_string()
    );
    target_text.sections[0].value = format!(
        "TARGETS LEFT: {}", targets_left
    );
    time_text.sections[0].value = format!(
        "TIME: {}", time_controller.get_time_left()
    );
}

// These methods return a bool depending on current state
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

pub fn in_won_state(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Won
}