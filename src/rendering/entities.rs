use crate::game::CameraController;
use bevy_rapier3d::prelude::*;
use bevy::{
    asset::LoadState, core_pipeline::Skybox, prelude::*, render::{
        render_resource::{
            TextureViewDescriptor,
            TextureViewDimension
        }, texture::CompressedImageFormats, view::NoFrustumCulling
    }, window::PrimaryWindow
};

#[derive(Component, Debug)]
pub struct PlayerController {
    pub velocity: Vec3,
    pub speed: f32,
    pub jump_height: f32,
    pub air_modifier: f32,
    pub crouch_modifier: f32,
    pub is_grounded: bool,
    pub is_crouched: bool,
}

#[derive(Component)]
pub struct GunController {
    pub magazine_size: usize,
    pub shoot: bool,
}

#[derive(Component, Resource)]
pub struct MapController {
    is_rotated: bool,
    scene_handle: Handle<Scene>
}

#[derive(Resource)]
pub struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            speed: 3.2,
            jump_height: 0.03,
            air_modifier: 1.0,
            crouch_modifier: 1.0,
            velocity: Vec3::ZERO,
            is_grounded: true,
            is_crouched: false,
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    primary_query: Query<&Window, With<PrimaryWindow>>
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
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // rungholt.glb & dm_lowpoly.gltf
        //visibility: Visibility::Hidden,
        ..Default::default()
    })
    .insert(RigidBody::Fixed)
    .insert(AsyncSceneCollider { ..default() })
    .insert(MapController { is_rotated: false, scene_handle: map.clone() } );

    let spawn_point = Vec3::new(-8.0, -1.0, 16.5); // CT-Spawn (-8.0, -1.0, 16.5)
    let primary_gun = asset_server.load("ak-47.glb#Scene0");

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
    )).with_children(|parent| {
        parent.spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.650, 0.0)),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: std::f32::consts::FRAC_PI_2 - 0.02,
                    ..Default::default()
                }),
                ..Default::default()
            },
            CameraController::default(),
            Skybox {
                image: skybox_handle.clone(),
                brightness: 1000.0
            },
        ));
        })
        //.with_children(|parent| {
        //    parent.spawn((
        //        SceneBundle {
        //            scene: primary_gun,
        //            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        //            ..Default::default()
        //        },
        //        GunController {
        //            shoot: false,
        //            magazine_size: 25
        //        }
        //    ));
        //})
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
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: crosshair_handle,
                    ..default()
                },
                style: Style {
                    top: Val::Px((1440. - 24.0) / 2.0), // fix primary.height()
                    left: Val::Px((2560. - 24.0) / 2.0), // fix primary.width()
                    ..Default::default()
                },
                ..default()
            });
        });

    commands.insert_resource(MapController {
        is_rotated: false,
        scene_handle: map
    });

    commands.insert_resource(Cubemap {
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

pub fn load_cubemap(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
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