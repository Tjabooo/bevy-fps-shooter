use bevy::window::CursorGrabMode;
use crate::structs::MenuButtonAction;
use crate::structs::MenuEntity;
use crate::GameState;
use bevy::{
    prelude::*,
    app::AppExit
};


pub fn setup_main_menu(
    mut commands: Commands
) { 
    commands.spawn(Camera2dBundle::default()).insert(MenuEntity);
    
    let button_style: Style = Style {
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
    ).insert(MenuEntity)
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
        ).insert(MenuEntity);

        // play button
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            },
            MenuButtonAction::Play,
            MenuEntity
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
            ).insert(MenuEntity);
        });
        
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            },
            MenuButtonAction::Quit,
            MenuEntity
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
            ).insert(MenuEntity);
        });
    });
}

pub fn menu_interactions(
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
                    new_game_state.set(GameState::Start);
                }
                MenuButtonAction::GoToMainMenu => {
                    new_game_state.set(GameState::MainMenu);
                }
                MenuButtonAction::Resume => {
                    new_game_state.set(GameState::Playing);
                }
                MenuButtonAction::Quit => {
                    app_exit_event.send(AppExit);
                }
            }
        }
    }
}

pub fn setup_pause_menu(
    mut commands: Commands,
    mut window: Query<&mut Window>
) {
    let mut window = window.get_single_mut().unwrap();

    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;

    let button_style: Style = Style {
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
    ).insert(MenuEntity)
    // title
    .with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            },
            MenuButtonAction::Resume,
            MenuEntity
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "RESUME",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..Default::default()
                    }
                )
            ).insert(MenuEntity);
        });
        
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            },
            MenuButtonAction::GoToMainMenu,
            MenuEntity
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "MAIN MENU",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..Default::default()
                    }
                )
            ).insert(MenuEntity);
        });
    });
}