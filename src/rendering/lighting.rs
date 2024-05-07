// DirectionalLightBundle (Infinite parallel, top down) -> from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
// SpotLightBundle (Soft one-directional) -> from_xyz(0.0, 0.0, 0.0)
// PointLightBundle (Soft omni-directional) -> from_xyz(0.0, 0.0, 0.0)
/*Illuminance (lux)	Surfaces illuminated by
0.0001	Moonless, overcast night sky (starlight)
0.002	Moonless clear night sky with airglow
0.05–0.3	Full moon on a clear night
3.4	Dark limit of civil twilight under a clear sky
20–50	Public areas with dark surroundings
50	Family living room lights
80	Office building hallway/toilet lighting
100	Very dark overcast day
150	Train station platforms
320–500	Office lighting
400	Sunrise or sunset on a clear day.
1000	Overcast day; typical TV studio lighting
10,000–25,000	Full daylight (not direct sun)
32,000–100,000	Direct sunlight
*/

use bevy::prelude::*;
use crate::structs::GameEntity;

pub fn setup(mut commands: Commands) {
    // sun
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_3)),
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 1500.0,
            shadows_enabled: false,
            shadow_depth_bias: 0.0000001,
            shadow_normal_bias: 3.0
        },
        ..Default::default()
    }).insert(GameEntity);

    // lamps
    let lamp_positions = [
        Vec3::new(-9.0, -0.4, 17.7),
        Vec3::new(7.8, 0.0, 4.5),
        Vec3::new(9.75, 2.1, -1.447),
        Vec3::new(21.8, 2.1, -0.375),
        Vec3::new(-11.55, 1.3, -9.0),
        Vec3::new(-21.8, -0.85, -13.9)
    ];

    for position in lamp_positions.iter() {
        commands.spawn(PointLightBundle {
            transform: Transform::from_translation(*position),
            point_light: PointLight {
                intensity: 10000.0,
                ..Default::default()
            },
            ..Default::default()
        }).insert(GameEntity);
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.,
    });
}