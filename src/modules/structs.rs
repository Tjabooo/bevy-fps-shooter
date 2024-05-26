use bevy::prelude::*;
use crate::GameState;
use std::time::Duration;

#[derive(Component, Debug, Resource)]
pub struct PlayerController {
    pub spawn_point: Vec3,
    pub view_model: Vec3,
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
            spawn_point: Vec3::new(-9.0, -1.0, 16.5), // CT-Spawn
            view_model: Vec3::new(0.10, -0.22, 0.35),
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
            sensitivity: 0.00045,
        }
    }
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct LevelText;

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
pub struct TimeText;

#[derive(Component, Resource, Debug, Default)]
pub struct TimeController {
    pub time_left: Option<Timer>,
    pub level_1_time: f32,
    pub level_2_time: f32,
    pub level_3_time: f32,
    pub level_4_time: f32,
    pub level_5_time: f32
}

impl TimeController {
    pub fn set_timer(&mut self, duration: f32) {
        self.time_left = Some(Timer::new(
            Duration::from_secs_f32(duration),
            TimerMode::Once
        ));
    }

    pub fn run_timer(&mut self, delta_time: Duration) {
        if let Some(ref mut timer) = self.time_left {
            timer.tick(delta_time);
        }
    }

    pub fn is_finished(&self) -> bool {
        if let Some(ref timer) = self.time_left {
            timer.finished()
        } else {
            false
        }
    }

    pub fn get_time_left(&self) -> String {
        if let Some(ref timer) = self.time_left {
            let duration_left = timer.duration() - timer.elapsed();
            format!("{:.2}s", duration_left.as_secs_f32())
        } else {
            "No timer".to_string()
        }
    }

    pub fn default() -> Self {
        Self {
            time_left: None,
            level_1_time: 50.0,
            level_2_time: 50.0,
            level_3_time: 70.0,
            level_4_time: 80.0,
            level_5_time: 90.0
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
    pub level_2_pos: Vec<Vec3>,
    pub level_3_pos: Vec<Vec3>,
    pub level_4_pos: Vec<Vec3>,
    pub level_5_pos: Vec<Vec3>
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
                Vec3::new(-2.7770, 1.1772, 5.2681),
                Vec3::new(-2.6943, 0.9644, -0.8119),
                Vec3::new(-0.7965, -0.7379, 2.1551),
                Vec3::new(-0.8334, -0.7380, 8.4862)
            ],
            level_3_pos: vec![
                Vec3::new(1.8366, -0.7378, 6.8549),
                Vec3::new(-0.8564, -0.6526, 17.0824),
                Vec3::new(11.7847, 0.9646, 11.7717),
                Vec3::new(11.8357, 2.8061, 18.8277),
                Vec3::new(18.0843, 0.9646, 5.7995),
                Vec3::new(13.6402, 1.0901, 21.1543),
                Vec3::new(21.0215, 1.3902, 19.6036),
                Vec3::new(23.0519, 0.9646, 9.1037),
                Vec3::new(24.0151, 1.3902, 0.8389),
                Vec3::new(16.1834, 1.3902, -2.6661),
                Vec3::new(11.1210, 1.6030, 0.0262),
                Vec3::new(9.2560, -0.5250, 4.2158),
                Vec3::new(-0.4572, -0.7378, 4.8973),
                Vec3::new(2.1751, 0.9646, -9.7511),
                Vec3::new(-8.1157, 1.1774, -9.5717),
                Vec3::new(-6.9634, 0.9646, -15.0531),
                Vec3::new(-12.2266, 0.9646, -9.1360),
                Vec3::new(-12.0709, 0.9646, -2.1852),
                Vec3::new(-19.3793, 0.9646, -9.0469),
                Vec3::new(-27.9691, 1.8158, -3.9547),
                Vec3::new(-28.2681, 1.7674, 9.6386),
                Vec3::new(-18.7076, 0.9646, 11.9895),
                Vec3::new(-12.4635, 1.4966, 15.9092)
            ],
            level_4_pos: vec![
                Vec3::new(-0.8194, -0.7378, 8.5195),
                Vec3::new(1.8417, -0.7378, 6.9533),
                Vec3::new(-0.8339, -0.7378, 4.4565),
                Vec3::new(-0.8411, -0.7378, 2.1493),
                Vec3::new(2.9636, -0.525, 4.4072),
                Vec3::new(7.763, -0.525, 1.6883),
                Vec3::new(11.1173, -0.525, 3.4607),
                Vec3::new(9.8671, 0.5345, -1.2861),
                Vec3::new(11.7012, 1.603, 0.0692),
                Vec3::new(16.2427, 1.3902, -2.5623),
                Vec3::new(18.4266, 1.3902, 2.4782),
                Vec3::new(15.0755, 1.3902, -7.6254),
                Vec3::new(20.7092, 1.3902, -8.6108),
                Vec3::new(14.5865, 0.9646, -10.9152),
                Vec3::new(24.8156, 2.667, -23.1453),
                Vec3::new(15.1152, 2.2414, -16.5035),
                Vec3::new(14.8551, 2.667, -21.7773),
                Vec3::new(14.7174, 2.9725, -28.9628),
                Vec3::new(9.859, 3.0031, -29.0852),
                Vec3::new(6.7452, 2.667, -24.4628),
                Vec3::new(6.5197, 2.9918, -29.0397),
                Vec3::new(-9.1692, 0.9646, -27.3666),
                Vec3::new(1.8755, 0.9646, -23.5141),
                Vec3::new(1.8435, 0.9646, -16.0974),
                Vec3::new(4.6532, 0.9646, -11.7088),
                Vec3::new(-2.8621, 0.9646, -8.9558),
                Vec3::new(-14.3503, 0.9646, -13.097),
                Vec3::new(-12.1312, 0.9646, -8.9037),
                Vec3::new(-11.7984, 0.9646, -4.1887),
                Vec3::new(-16.4408, 0.9646, -5.2008),
                Vec3::new(-19.1068, 0.9646, -8.9071),
                Vec3::new(-23.9353, -1.589, -13.2769),
                Vec3::new(-27.2306, 1.8158, -9.8214),
                Vec3::new(-27.5919, 1.8158, -3.9932),
                Vec3::new(-22.5547, 0.9646, 1.0527),
                Vec3::new(-27.9728, 1.6936, 9.6746),
                Vec3::new(-28.0014, 1.7008, 14.4763),
                Vec3::new(-21.9798, 1.463, 16.7881),
                Vec3::new(-19.7779, 2.667, 24.8226),
                Vec3::new(-18.6064, 2.2414, 17.8125),
                Vec3::new(-8.7551, 2.2414, 20.2009),
                Vec3::new(-10.9351, 2.2414, 13.329),
                Vec3::new(-10.9078, 2.2414, 8.9241),
                Vec3::new(-10.8079, 0.9646, 6.2139),
                Vec3::new(-5.0401, 0.9646, 3.4032),
                Vec3::new(-2.7286, 1.1774, 5.2457)
            ],
            level_5_pos: vec![
                Vec3::new(-0.6412, -0.7378, 7.1164),
                Vec3::new(1.667, -0.7378, 7.001),
                Vec3::new(-0.7431, -0.4742, 16.9093),
                Vec3::new(6.7045, 0.1932, 19.1925),
                Vec3::new(11.7762, 2.434, 19.1522),
                Vec3::new(11.5851, 0.9646, 12.0045),
                Vec3::new(13.9705, 0.9646, 10.2141),
                Vec3::new(17.7754, 0.9646, 6.1688),
                Vec3::new(17.6912, 2.3839, 18.0894),
                Vec3::new(13.7182, 1.079, 21.196),
                Vec3::new(17.6703, 0.9647, 20.9911),
                Vec3::new(21.6996, 1.3902, 19.7469),
                Vec3::new(22.5006, 1.3902, 25.0933),
                Vec3::new(19.6402, 0.9646, 9.2387),
                Vec3::new(23.0919, 0.9646, 9.459),
                Vec3::new(18.0257, 1.3902, 2.3925),
                Vec3::new(23.8587, 1.3902, 0.7156),
                Vec3::new(16.3217, 1.3902, -2.5154),
                Vec3::new(13.594, 1.3902, 0.8169),
                Vec3::new(14.8941, 1.3902, -7.1904),
                Vec3::new(21.0214, 1.3902, -7.7979),
                Vec3::new(22.1341, 0.9646, -11.4787),
                Vec3::new(14.7329, 0.9646, -9.8902),
                Vec3::new(19.4653, 1.7562, -20.6927),
                Vec3::new(24.3389, 2.667, -23.4303),
                Vec3::new(21.6162, 2.9584, -28.9062),
                Vec3::new(15.1401, 2.2414, -16.7951),
                Vec3::new(8.8826, 2.8898, -20.369),
                Vec3::new(6.8325, 2.667, -24.448),
                Vec3::new(6.8238, 3.013, -29.1245),
                Vec3::new(-2.515, 1.8341, -29.0815),
                Vec3::new(-9.1304, 0.9646, -27.3654),
                Vec3::new(-6.0983, 0.9646, -22.5966),
                Vec3::new(1.8303, 0.9646, -23.4483),
                Vec3::new(1.8022, 0.9646, -16.2652),
                Vec3::new(3.5292, 0.9646, -11.7815),
                Vec3::new(1.6, 0.9646, -7.7739),
                Vec3::new(3.2915, 0.9646, -9.3833),
                Vec3::new(-2.051, 0.9646, -11.5675),
                Vec3::new(-8.0399, 1.1774, -9.7182),
                Vec3::new(-6.3889, 0.9646, -12.5047),
                Vec3::new(-14.1784, 0.9646, -13.0022),
                Vec3::new(-12.0466, 0.9646, -9.2751),
                Vec3::new(-11.9041, 0.9646, -4.5474),
                Vec3::new(-14.3086, 0.9646, -0.2996),
                Vec3::new(-19.3886, 0.9646, -9.2222),
                Vec3::new(-23.4931, -1.589, -13.9355),
                Vec3::new(-27.2601, 1.8158, -9.7836),
                Vec3::new(-27.3486, 1.8158, -6.9392),
                Vec3::new(-27.4452, 1.8158, -3.8519),
                Vec3::new(-22.3242, 0.9646, 2.5074),
                Vec3::new(-25.4838, 1.0714, 2.6171),
                Vec3::new(-28.1668, 1.7421, 9.6558),
                Vec3::new(-28.1497, 1.7378, 14.44),
                Vec3::new(-20.0641, 0.9646, 13.2482),
                Vec3::new(-25.6465, 2.8145, 24.7135),
                Vec3::new(-18.9272, 3.5182, 24.465),
                Vec3::new(-18.958, 2.2414, 18.6402),
                Vec3::new(-21.0479, 2.2414, 15.6339),
                Vec3::new(-13.7131, 2.2415, 19.5801),
                Vec3::new(-8.9971, 2.2414, 16.7124),
                Vec3::new(-11.1697, 2.2414, 11.7399),
                Vec3::new(-10.9501, 2.2414, 8.846),
                Vec3::new(-10.9189, 0.9646, 5.7322),
                Vec3::new(-7.2235, 0.9646, 3.2609),
                Vec3::new(-2.6392, 1.1774, 5.2141),
                Vec3::new(-2.1854, 0.9646, -3.254),
                Vec3::new(-0.8358, -0.7378, 2.1606),
                Vec3::new(1.601, -0.7378, 5.0437),
                Vec3::new(3.239, -0.525, 1.6187),
                Vec3::new(3.2527, -0.525, 4.2351),
                Vec3::new(7.7485, -0.525, 1.6004),
                Vec3::new(11.2624, -0.525, 3.5596),
                Vec3::new(9.8675, 0.5495, -1.3613),
                Vec3::new(10.9497, 1.603, 0.9789)
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

#[derive(Component)]
pub struct TextEntity;

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

#[derive(Component)]
pub struct StartButton;

#[derive(Resource)]
pub struct LastState {
    pub state: Option<GameState>
}

impl Default for LastState {
    fn default() -> Self {
        Self {
            state: None
        }
    }
}

#[derive(Component)]
pub struct MapImage;

#[derive(Resource)]
pub struct PlayerEntity {
    pub entity: Option<Entity>
}

impl Default for PlayerEntity {
    fn default() -> Self {
        Self {
            entity: None
        }
    }
}