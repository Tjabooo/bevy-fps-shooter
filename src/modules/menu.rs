use crate::structs::MenuButtonAction;
use crate::structs::MenuCameraController;
use crate::GameState;
use bevy::{
    prelude::*,
    app::AppExit
};

pub fn setup(
    mut commands: Commands,
    mut menu_camera_controller: Query<
) {
    commands.spawn((
        Camera2dBundle {
            
        }
    ))

    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        }
    )
    // title
    .with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "VALORANT 2.0",
                TextStyle {
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..Default::default()
                }
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(50.0)),
                ..Default::default()
            })
        );

        // play button
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            },
            MenuButtonAction::Play
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "PLAY",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..Default::default()
                    }
                )
            );
        });
        
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            },
            MenuButtonAction::Quit
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "QUIT",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..Default::default()
                    }
                )
            );
        });
    });
}

pub fn menu_interaction(
    mut commands: Commands,
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_event: ResMut<Events<AppExit>>,
    mut new_game_state: ResMut<NextState<GameState>>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    new_game_state.set(GameState::Playing);
                    commands.entity(camera_entity).despawn();
                }
                MenuButtonAction::Quit => {
                    app_exit_event.send(AppExit);
                }
            }
        }
    }
}