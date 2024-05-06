use bevy::prelude::*;

#[derive(Component, Debug, Resource)]
pub struct PlayerController {
    pub velocity: Vec3,
    pub speed: f32,
    pub jump_height: f32,
    pub crouch_modifier: f32,
    pub is_grounded: bool,
    pub is_crouched: bool,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            speed: 3.2,
            jump_height: 0.11,
            crouch_modifier: 1.0,
            velocity: Vec3::ZERO,
            is_grounded: true,
            is_crouched: false,
        }
    }
}

#[derive(Component, Debug)]
pub struct EnemyController {
    pub health: i32,
}

#[derive(Component, Resource)]
pub struct GunController {
    pub shooting: bool,
    pub bullet_delay: Option<Timer>,
    pub just_pressed: bool,
    pub is_rotated: bool,
    pub model_handle: Option<Handle<Scene>>
}

impl Default for GunController {
    fn default() -> Self {
        Self {
            shooting: false,
            bullet_delay: None,
            just_pressed: false,
            is_rotated: false,
            model_handle: None
        }
    }
}

#[derive(Component, Resource)]
pub struct MapController {
    pub is_rotated: bool,
    pub scene_handle: Option<Handle<Scene>>
}

impl Default for MapController {
    fn default() -> Self {
        Self {
            is_rotated: false,
            scene_handle: None
        }
    }
}

#[derive(Resource)]
pub struct CubemapController {
    pub is_loaded: bool,
    pub image_handle: Option<Handle<Image>>
}

impl Default for CubemapController {
    fn default() -> Self {
        Self {
            is_loaded: false,
            image_handle: None
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

#[derive(Resource)]
pub struct AudioController {
    pub ambience_handle: Option<Handle<AudioSource>>,
    pub gunshot_handle: Option<Handle<AudioSource>>
}

impl Default for AudioController {
    fn default() -> Self {
        Self {
            ambience_handle: None,
            gunshot_handle: None
        }
    }
}

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
    Resume,
    GoToMainMenu
}

#[derive(Component)]
pub struct MenuEntity;

#[derive(Component)]
pub struct GameEntity;