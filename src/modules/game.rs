use crate::entities::PlayerController;
use bevy::{
    app::AppExit,
    prelude::*,
    input::mouse::MouseMotion,
    diagnostic::{
        DiagnosticsStore,
        FrameTimeDiagnosticsPlugin
    }, 
};

#[derive(Component, Debug)]
pub struct CameraController {
    pub pitch: f32,
    pub yaw: f32,
    pub sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.00025,
        }
    }
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
struct ColorText;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMonoNLNerdFont-Regular.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 30.0,
                    color: Color::GOLD,
                    ..default()
                }
            } else {
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMonoNLNerdFont-Regular.ttf"),
                    font_size: 30.0,
                    ..default()
                }
            }),
        ]),
        FpsText,
    ));    
}

pub fn update(
    key: ResMut<ButtonInput<KeyCode>>,
    mut app_exit_event: ResMut<Events<AppExit>>,
    mut player_query: Query<&mut Transform, With<PlayerController>>,
    mut camera_query: Query<(&mut CameraController, &mut Transform), (With<Camera>, Without<PlayerController>)>,
    mut mouse_motion_events: EventReader<MouseMotion>
) {
    if key.just_pressed(KeyCode::Escape) {
        app_exit_event.send(AppExit);
    }

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

pub fn diagnostics(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}