use crate::structs::{
    CameraController,
    PlayerController,
    EnemyController,
    GunController,
    MapController,
    CubemapController
};
use bevy_rapier3d::prelude::*;
use bevy_scene_hook::{
    HookedSceneBundle,
    SceneHook
};
use bevy::{
    prelude::*,
    asset::LoadState,
    core_pipeline::Skybox,
    window::PrimaryWindow,
    render::{
        view::NoFrustumCulling,
        texture::CompressedImageFormats,
        render_resource::{
            TextureViewDescriptor,
            TextureViewDimension
        },
    }, 
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let Ok(primary) = primary_query.get_single() else
    {
        return;
    };

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
    .insert(MapController { is_rotated: false, scene_handle: map.clone() } );

    let spawn_point = Vec3::new(-8.0, -1.5, 16.5); // CT-Spawn (-8.0, -1.0, 16.5)
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
        VisibilityBundle {
            inherited_visibility: InheritedVisibility::VISIBLE,
            ..Default::default()
        }
    ))
    .with_children(|parent| {
        parent.spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.650, 0.0)),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: std::f32::consts::FRAC_PI_2 - 0.1,
                    ..Default::default()
                }),
                ..Default::default()
            },
            CameraController::default(),
            Skybox {
                image: skybox_handle.clone(),
                brightness: 1000.0
            },
            VisibilityBundle {
                inherited_visibility: InheritedVisibility::VISIBLE,
                ..Default::default()
            }
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
                    bullet_delay: Timer::from_seconds(0.1, TimerMode::Repeating),
                    just_pressed: false,
                    is_rotated: false,
                    model_handle: gun_handle.clone(),
                },
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

    let texture_handle = asset_server.load("textures/default_texture.png");
    //let material = materials.add(texture_handle.into());
    let material = materials.add(StandardMaterial { base_color_texture: Some(texture_handle.clone()), ..Default::default() }); 
    let mesh = meshes.add(Sphere { radius: 0.2, ..Default::default() });

    commands
        .spawn((
            PbrBundle {
                mesh: mesh,
                material: material,
                transform: Transform::from_translation(
                    Vec3::new(
                        spawn_point.x,
                        spawn_point.y + 0.5,
                        spawn_point.z - 2.0
                    )
                ),
                ..Default::default()
            },
            AsyncCollider { ..Default::default() },
            RigidBody::Fixed,
            EnemyController {
                health: 10
            }
        ),
    );

    // headcrab zombie enemy model
    //let enemy = asset_server.load("enemy.glb#Scene0");
    //
    //commands
    //    .spawn((
    //        SceneBundle {
    //            scene: enemy,
    //            transform: Transform::from_translation(
    //                Vec3::new(
    //                    spawn_point.x,
    //                    spawn_point.y,
    //                    spawn_point.z - 2.0
    //                )
    //            ),
    //            ..Default::default()
    //        },
    //        AsyncSceneCollider { ..Default::default() },
    //        RigidBody::Dynamic,
    //        Sleeping::disabled(),
    //        LockedAxes::ROTATION_LOCKED,
    //        EnemyController {
    //            health: 100
    //        },
    //    ));

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
            });

    commands.insert_resource(MapController {
        is_rotated: false,
        scene_handle: map
    });

    commands.insert_resource(GunController {
        shooting: false,
        bullet_delay: Timer::from_seconds(0.1, TimerMode::Repeating),
        just_pressed: false,
        is_rotated: false,
        model_handle: gun_handle
    });

    commands.insert_resource(CubemapController {
        is_loaded: false,
        image_handle: skybox_handle
    });
}

pub fn rotate_map(
    mut query: Query<&mut Transform, With<MapController>>,
    mut map_controller: ResMut<MapController>,
    asset_server: Res<AssetServer>,
) {
    if !map_controller.is_rotated && asset_server.load_state(&map_controller.scene_handle) == LoadState::Loaded {
        for mut transform in query.iter_mut() {
            transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));
            map_controller.is_rotated = true;
        }
    }
}

pub fn rotate_gun(
    mut query: Query<&mut Transform, With <GunController>>,
    mut gun_controller: ResMut<GunController>,
    asset_server: Res<AssetServer>
) {
    if !gun_controller.is_rotated && asset_server.load_state(&gun_controller.model_handle) == LoadState::Loaded {
        for mut transform in query.iter_mut() {
            transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));
            gun_controller.is_rotated = true;
        }
    }
}

pub fn load_cubemap(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<CubemapController>,
    mut skyboxes: Query<&mut Skybox>
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone()
        }

        cubemap.is_loaded = true;
    }
}