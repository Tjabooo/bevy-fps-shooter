use bevy_rapier3d::prelude::*;
use crate::{
    GameState,
    structs::{
        CameraController,
        CubemapController,
        TargetController,
        GunController,
        MapController,
        PlayerController,
        MenuEntity,
        GameEntity,
        TextEntity,
        EntityHandler,
        PlayerEntity
    }
};
use bevy_scene_hook::{
    HookedSceneBundle,
    SceneHook
};
use bevy::{
    prelude::*,
    asset::LoadState,
    core_pipeline::Skybox,
    render::{
        texture::CompressedImageFormats,
        view::NoFrustumCulling,
        render_resource::{
            TextureViewDescriptor,
            TextureViewDimension
        },
    } 
};

// Loads all the entities needed for the game
pub fn load_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let map_handle = asset_server.load("de_dust2.glb#Scene0");
    let gun_handle = asset_server.load("cs1.6_ak-47.glb#Scene0");
    let crosshair_handle = asset_server.load("textures/crosshair.png");
    let target_texture_handle = asset_server.load("textures/default_texture.png");
    let text_font_handle = asset_server.load("fonts/JetBrainsMonoNLNerdFont-Regular.ttf");

    commands.insert_resource(EntityHandler {
        map_handle: Some(map_handle),
        gun_handle: Some(gun_handle),
        crosshair_handle: Some(crosshair_handle),
        target_texture_handle: Some(target_texture_handle),
        text_font_handle: Some(text_font_handle)
    });
}

// Sets up the game entities
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entity_handler: Res<EntityHandler>,
    player_controller: Res<PlayerController>,
    mut player_entity: ResMut<PlayerEntity>
) {
    //skybox
    const CUBEMAP: &[(&str, CompressedImageFormats)] = &[
        (
            "textures/skybox_cubemap.png",
            CompressedImageFormats::NONE,
        )
    ];
    let skybox_handle = asset_server.load(CUBEMAP[0].0);

    // map
    commands.spawn(SceneBundle {
        scene: entity_handler.map_handle.clone().expect(""),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    })
    .insert(RigidBody::Fixed)
    .insert(AsyncSceneCollider { ..default() })
    .insert(MapController { is_rotated: false, scene_handle: entity_handler.map_handle.clone() } )
    .insert(GameEntity);

    // player
    let player_entity_id = commands.spawn((
        PlayerController { ..Default::default() },
        RigidBody::Dynamic,
        GravityScale(0.9),
        Sleeping::disabled(),
        Collider::capsule(Vec3::ZERO, Vec3::new(0.0, 0.450, 0.0), 0.1),
        LockedAxes::ROTATION_LOCKED,
        Ccd { enabled: true },
        VisibilityBundle::default(),
        GameEntity
    ))
    .with_children(|parent| {
        parent.spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.650, 0.0)),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: std::f32::consts::FRAC_PI_2 - 0.1,
                    near: 0.01,
                    ..Default::default()
                }),
                ..Default::default()
            },
            CameraController::default(),
            Skybox {
                image: skybox_handle.clone(),
                brightness: 1000.0
            },
            VisibilityBundle::default()
        )).with_children(|parent| {
            parent.spawn((
                HookedSceneBundle {
                    scene: SceneBundle {
                        scene: entity_handler.gun_handle.clone().expect(""),
                        transform: Transform::from_translation(player_controller.view_model),
                        ..Default::default()
                    },
                    hook: SceneHook::new(|entity, commands| {
                        if entity.get::<Handle<Mesh>>().is_some() {
                            commands.insert(NoFrustumCulling);
                        }
                    })
                },
                GunController {
                    shooting: false,
                    bullet_delay: Some(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    just_pressed: false,
                    is_rotated: false,
                    model_handle: entity_handler.gun_handle.clone(),
                    play_audio: false
                },
            ));
        });
        })
        .insert(
            TransformBundle::from(
                Transform::from_xyz(
                    player_controller.spawn_point.x,
                    player_controller.spawn_point.y,
                    player_controller.spawn_point.z
                )
            )
        ).id();

    // sets the player entity
    player_entity.entity = Some(player_entity_id);

    // crosshair
    commands.spawn(ImageBundle {
        image: UiImage {
            texture: entity_handler.crosshair_handle.clone().expect(""),
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Px(24.0),
            height: Val::Px(24.0),
            left: Val::Percent(50.0),
            bottom: Val::Percent(50.0),
            margin: UiRect {
                left: Val::Px(-12.0),
                bottom: Val::Px(-12.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }).insert(GameEntity);

    commands.insert_resource(MapController {
        is_rotated: false,
        scene_handle: entity_handler.map_handle.clone()
    });

    commands.insert_resource(GunController {
        shooting: false,
        bullet_delay: Some(Timer::from_seconds(0.1, TimerMode::Repeating)),
        just_pressed: false,
        is_rotated: false,
        model_handle: entity_handler.gun_handle.clone(),
        play_audio: false
    });

    commands.insert_resource(CubemapController {
        is_loaded: false,
        image_handle: Some(skybox_handle)
    });
}

// Spawns the start text
pub fn spawn_start_text(
    mut commands: Commands,
    text_entity_query: Query<&TextEntity>,
    entity_handler: Res<EntityHandler>
) {
    if text_entity_query.iter().count() == 0 {
        commands.spawn(
            TextBundle::from_section(
                "Shoot the 'Start' button to begin!",
                TextStyle {
                    font: entity_handler.text_font_handle.clone().expect(""),
                    font_size: 30.0,
                    ..Default::default()
                }
            ).with_style(Style {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                top: Val::Percent(10.0),
                ..Default::default()
            })
        ).insert((TextEntity, GameEntity));
    }
}

// Spawns the win text
pub fn spawn_win_text(
    mut commands: Commands,
    entity_handler: Res<EntityHandler>
) {
    commands.spawn(
        TextBundle::from_section(
            "You won! Beat your personal best by trying again!",
            TextStyle {
                font: entity_handler.text_font_handle.clone().expect(""),
                font_size: 30.0,
                ..Default::default()
            }
        ).with_style(Style {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            top: Val::Percent(10.0),
            ..Default::default()
        })
    ).insert((TextEntity, GameEntity));
}

// Spawns the fail text
pub fn spawn_fail_text(
    mut commands: Commands,
    entity_handler: Res<EntityHandler>
) {
    commands.spawn(
        TextBundle::from_section(
            "You failed. Shoot the 'Start' button to try again.",
            TextStyle {
                font: entity_handler.text_font_handle.clone().expect(""),
                font_size: 30.0,
                ..Default::default()
            }
        ).with_style(Style {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            top: Val::Percent(10.0),
            ..Default::default()
        })
    ).insert((TextEntity, GameEntity));
}

// Rotates the map
pub fn rotate_map(
    mut query: Query<&mut Transform, With<MapController>>,
    mut map_controller: ResMut<MapController>,
    asset_server: Res<AssetServer>,
    mut change_state: ResMut<NextState<GameState>>
) {
    if let Some(scene_handle) = &map_controller.scene_handle {
        if !map_controller.is_rotated && asset_server.load_state(scene_handle) == LoadState::Loaded {
            for mut transform in query.iter_mut() {
                transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));
                map_controller.is_rotated = true;
                change_state.set(GameState::Start);
            }
        }
    }
}

// Rotates the gun
pub fn rotate_gun(
    mut query: Query<&mut Transform, With <GunController>>,
    mut gun_controller: ResMut<GunController>,
    asset_server: Res<AssetServer>
) {
    if let Some(model_handle) = &gun_controller.model_handle {
        if !gun_controller.is_rotated && asset_server.load_state(model_handle) == LoadState::Loaded {
            for mut transform in query.iter_mut() {
                transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));
                gun_controller.is_rotated = true;
            }
        }
    };
}

// Loads the skybox cubemap
pub fn load_cubemap(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<CubemapController>,
    mut skyboxes: Query<&mut Skybox>
) {
    if let Some(image_handle) = &cubemap.image_handle {
        if !cubemap.is_loaded && asset_server.load_state(image_handle) == LoadState::Loaded {
            let image = images.get_mut(image_handle).unwrap();
            if image.texture_descriptor.array_layer_count() == 1 {
                image.reinterpret_stacked_2d_as_array(image.height() / image.width());
                image.texture_view_descriptor = Some(TextureViewDescriptor {
                    dimension: Some(TextureViewDimension::Cube),
                    ..Default::default()
                });
            }
            for mut skybox in &mut skyboxes {
                skybox.image = image_handle.clone()
            }
            cubemap.is_loaded = true;
        }
    }
}

// These methods are used to despawn entities
pub fn despawn_menu_entities(
    mut commands: Commands,
    menu_entity_query: Query<Entity, With<MenuEntity>>
) {
    for menu_entity in menu_entity_query.iter() {
        commands.entity(menu_entity).despawn();
    }
}

pub fn despawn_game_entities(
    mut commands: Commands,
    game_entity_query: Query<Entity, With<GameEntity>>
) {
    for game_entity in game_entity_query.iter() {
        commands.entity(game_entity).despawn_recursive();
    }
}

pub fn despawn_text_entities(
    mut commands: Commands,
    text_entity_query: Query<Entity, With<TextEntity>>
) {
    for text_entity in text_entity_query.iter() {
        commands.entity(text_entity).despawn_recursive();
    }
}

pub fn despawn_targets(
    mut commands: Commands,
    target_entity_query: Query<Entity, With<TargetController>>
) {
    for target_entity in target_entity_query.iter() {
        commands.entity(target_entity).despawn_recursive();
    }
}