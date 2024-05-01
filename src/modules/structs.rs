use bevy::prelude::*;

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

#[derive(Component, Debug)]
pub struct EnemyController {
    pub health: i32,
}

#[derive(Component, Resource)]
pub struct GunController {
    pub shooting: bool,
    pub bullet_delay: Timer,
    pub just_pressed: bool,
    pub is_rotated: bool,
    pub model_handle: Handle<Scene>
}

#[derive(Component, Resource)]
pub struct MapController {
    pub is_rotated: bool,
    pub scene_handle: Handle<Scene>
}

#[derive(Resource)]
pub struct CubemapController {
    pub is_loaded: bool,
    pub image_handle: Handle<Image>
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
pub struct Ambience;