use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // map
    let scene = asset_server.load("de_dust2.gltf#Scene0");
    commands.spawn(SceneBundle {
        scene: scene,
        transform: Transform::from_xyz(0.0, -1007.2, -1205.0), // de_dust2.glb/gltf
        //transform: Transform::from_xyz(0.0, 0.0, 0.0), // rungholt.glb & dm_lowpoly.gltf
        //visibility: Visibility::Hidden,
        ..Default::default()
    })
    .insert(RigidBody::Fixed)
    .insert(AsyncSceneCollider { ..default() });
    
    // ground
    //commands
    //    .spawn(Collider::cuboid(100.0, 0.1, 100.0))
    //    .insert(RigidBody::Fixed);
}