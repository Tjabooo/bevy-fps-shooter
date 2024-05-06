use crate::structs::{
    CameraController,
    CubemapController,
    EnemyController,
    GunController,
    MapController,
    PlayerController,
    MenuEntity,
    GameEntity
};
use crate::GameState;
use bevy_rapier3d::prelude::*;
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

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
    let map = asset_server.load("de_dust2.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: map.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        //visibility: Visibility::Hidden,
        ..Default::default()
    })
    .insert(RigidBody::Fixed)
    .insert(AsyncSceneCollider { ..default() })
    .insert(MapController { is_rotated: false, scene_handle: Some(map.clone()) } )
    .insert(GameEntity);

    let spawn_point = Vec3::new(-8.0, -1.0, 16.5); // CT-Spawn (-8.0, -1.0, 16.5)
    let view_model = Vec3::new(0.10, -0.22, 0.35);
    let gun_handle = asset_server.load("cs1.6_ak-47.glb#Scene0");

    // player
    commands.spawn((
        PlayerController { ..Default::default() },
        RigidBody::Dynamic,
        GravityScale(0.9),
        Sleeping::disabled(),
        //Collider::cuboid(0.15, 0.500, 0.15),
        Collider::capsule(Vec3::ZERO, Vec3::new(0.0, 0.450, 0.0), 0.1),
        LockedAxes::ROTATION_LOCKED,
        //ActiveEvents::COLLISION_EVENTS,
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
            VisibilityBundle::default(),
            GameEntity
        )).with_children(|parent| {
            parent.spawn((
                HookedSceneBundle {
                    scene: SceneBundle {
                        scene: gun_handle.clone(),
                        transform: Transform::from_translation(view_model),
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
                    model_handle: Some(gun_handle.clone()),
                },
                GameEntity
            ));
        });
        })
        .insert(
            TransformBundle::from(
                Transform::from_xyz(
                    spawn_point.x,
                    spawn_point.y,
                    spawn_point.z
                )
            )
        );

    //println!("primary: {}", primary.width());
    let crosshair_handle = asset_server.load("textures/crosshair.png");

    // crosshair
    commands
       .spawn(ImageBundle {
                image: UiImage {
                    texture: crosshair_handle,
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
        scene_handle: Some(map)
    });

    commands.insert_resource(GunController {
        shooting: false,
        bullet_delay: Some(Timer::from_seconds(0.1, TimerMode::Repeating)),
        just_pressed: false,
        is_rotated: false,
        model_handle: Some(gun_handle)
    });

    commands.insert_resource(CubemapController {
        is_loaded: false,
        image_handle: Some(skybox_handle)
    });
}

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let texture_handle = asset_server.load("textures/default_texture.png");
    let material = materials.add(StandardMaterial { base_color_texture: Some(texture_handle.clone()), ..Default::default() }); 
    let mesh = meshes.add(Sphere { radius: 0.1 });

    let level_1_positions = [
        Vec3::new(-18.23, 2.25, 16.47),
        Vec3::new(-19.78, 2.72, 24.65),
        Vec3::new(-28.2, 1.85, 9.7),
        Vec3::new(-28.17, 1.93, -3.75),
        Vec3::new(-13.42, 1.115, -0.3),
        Vec3::new(-13.9, 1.114, -10.5),
        Vec3::new(-8.0, 1.23, -9.55),
        Vec3::new(-2.69, 1.015, -6.33),
        Vec3::new(1.69, -0.72, 4.9),
        Vec3::new(-0.84, -0.69, 9.2)
    ];

    for enemy_position in level_1_positions.iter() {
        commands
            .spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    transform: Transform::from_translation(*enemy_position),
                    ..Default::default()
                },
                AsyncCollider { ..Default::default() },
                RigidBody::Fixed,
                EnemyController {
                    health: 1
                },
                GameEntity
            ));
    }
}

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
                change_state.set(GameState::Playing);
            }
        }
    }
}

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
        commands.entity(game_entity).despawn();
    }
}