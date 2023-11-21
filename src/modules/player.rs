use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Player {
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
    pub speed: f32,
    pub jump_height: f32,
    pub is_grounded: bool
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            speed: 2.5,
            jump_height: 2.5,
            velocity: Vec3::ZERO,
            is_grounded: true
        }
    }
}

pub fn setup(mut commands: Commands) {
    let spawn_point = Vec3::new(0.0, 1.0, 0.0);

    // player
    commands.spawn(Player {
        ..Default::default()
    })
    .insert(Camera3dBundle { ..Default::default() } )
    .insert(RigidBody::Dynamic)
    .insert(GravityScale(0.5))
    .insert(Sleeping::disabled())
    .insert(Collider::capsule(Vec3::ZERO, Vec3::Y * 0.4, 0.2))
    .insert(Restitution::coefficient(0.0))
    .insert(TransformBundle::from(Transform::from_xyz(spawn_point.x, spawn_point.y, spawn_point.z)))
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Ccd { enabled: true });
}

pub fn mouse_callback(mut query: Query<(&mut Player, &mut Transform), With<Camera>>, mut mouse_motion_events: EventReader<MouseMotion>) {
    for (mut player, mut transform) in query.iter_mut() {
        for event in mouse_motion_events.read() {
            const SENSITIVITY: f32 = 0.001;
            const MAX_VERTICAL_ANGLE: f32 = std::f32::consts::FRAC_PI_2 - 0.02;
            
            player.pitch += -event.delta.y * SENSITIVITY;
            player.yaw += -event.delta.x * SENSITIVITY;
            
            player.pitch = player.pitch.clamp(-MAX_VERTICAL_ANGLE, MAX_VERTICAL_ANGLE);
            
            transform.rotation = Quat::from_axis_angle(Vec3::Y, player.yaw) * Quat::from_axis_angle(Vec3::X, player.pitch);
        }
    }
}