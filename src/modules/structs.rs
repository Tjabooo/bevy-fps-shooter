use bevy::{log::Level, prelude::*};

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

#[derive(Component, Debug, Resource)]
pub struct TargetController {
    pub health: i32,
}

impl Default for TargetController {
    fn default() -> Self {
        Self {
            health: 1
        }
    }
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
pub struct TargetText {
    pub targets_left: Option<usize>
}

impl Default for TargetText {
    fn default() -> Self {
        Self {
            targets_left: None
        }
    }
}

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

#[derive(Resource)]
pub struct LevelController {
    pub level_1_pos: Vec<Vec3>,
    pub level_2_pos: Vec<Vec3>
}

impl Default for LevelController {
    fn default() -> Self {
        Self {
            level_1_pos: vec![
                Vec3::new(-28.1820, 1.7458, 9.6431),
                Vec3::new(-27.9758, 1.7657, -3.8952),
                Vec3::new(-18.1986, 0.9645, -10.1506),
                Vec3::new(-13.4816, 0.9645, -0.2423),
                Vec3::new(-14.2918, 0.9625, -8.6672),
                Vec3::new(-8.0804, 1.1772, -9.7454),
                Vec3::new(-0.7965, -0.7379, 2.1551),
                Vec3::new(-0.8334, -0.7380, 8.4862)
            ],
            level_2_pos: vec![
                Vec3::new(-28.2415, 1.7605, 14.4738),
                Vec3::new(-19.6705, 2.6669, 24.6628),
                Vec3::new(-19.7269, 2.2412, 15.9171),
                Vec3::new(-8.7544, 2.2123, 20.1367),
                Vec3::new(-10.6621, 2.2412, 9.1706),
                Vec3::new(-11.0602, 0.9644, 7.4012),
                Vec3::new(-2.7770, 0.4772, 5.2681),
                Vec3::new(-2.6943, 0.2644, -0.8119),
                Vec3::new(-0.7965, -0.7379, 2.1551),
                Vec3::new(-0.8334, -0.7380, 8.4862)
            ]
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

#[derive(Resource)]
pub struct EntityHandler {
    pub map_handle: Option<Handle<Scene>>,
    pub gun_handle: Option<Handle<Scene>>,
    pub crosshair_handle: Option<Handle<Image>>,
    pub target_texture_handle: Option<Handle<Image>>,
}

impl Default for EntityHandler {
    fn default() -> Self {
        Self {
            map_handle: None,
            gun_handle: None,
            crosshair_handle: None,
            target_texture_handle: None
        }
    }
}