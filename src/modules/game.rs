use bevy::{
    prelude::*,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    app::AppExit
};

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

pub fn update(key: ResMut<Input<KeyCode>>, mut app_exit_event: ResMut<Events<AppExit>>, diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    if key.just_pressed(KeyCode::Escape) {
        app_exit_event.send(AppExit);
    }   

    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}