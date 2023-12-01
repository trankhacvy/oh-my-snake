use bevy::{app::AppExit, prelude::*};

use crate::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup);
        app.add_systems(
            Update,
            handle_buttons_interaction.run_if(in_state(GameState::GameOver)),
        );
        app.add_systems(OnExit(GameState::GameOver), cleanup);
    }
}

#[derive(Component)]
struct PlayAgain;

#[derive(Component)]
struct Quit;

#[derive(Component)]
struct GameOverUI;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(40.0),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|children| {
            // text
            children.spawn(
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "Gameover",
                    TextStyle {
                        font: asset_server.load("fonts/KnightWarrior.otf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Center),
            );

            // actions
            children
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.),
                        column_gap: Val::Px(40.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    // play again
                    children
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(160.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                background_color: Color::VIOLET.into(),
                                ..Default::default()
                            },
                            PlayAgain,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play Again",
                                TextStyle {
                                    font_size: 24.0,
                                    font: asset_server.load("fonts/KnightWarrior.otf"),
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        });

                    // quit
                    children
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(160.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                background_color: Color::ORANGE_RED.into(),
                                ..Default::default()
                            },
                            Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Quit",
                                TextStyle {
                                    font_size: 24.0,
                                    font: asset_server.load("fonts/KnightWarrior.otf"),
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

fn handle_buttons_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayAgain>,
            Option<&Quit>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, maybe_btn_play_again, maybe_btn_quit) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(_) = maybe_btn_play_again {
                    next_state.set(GameState::Game);
                } else if let Some(_) = maybe_btn_quit {
                    app_exit_events.send(AppExit);
                }
            }
            Interaction::Hovered => {
                if let Some(_) = maybe_btn_play_again {
                    *color = Color::rgba(0.93, 0.51, 0.93, 0.5).into();
                } else if let Some(_) = maybe_btn_quit {
                    *color = Color::rgba(1.0, 0.27, 0.0, 0.5).into();
                }
            }
            Interaction::None => {
                if let Some(_) = maybe_btn_play_again {
                    *color = Color::VIOLET.into();
                } else if let Some(_) = maybe_btn_quit {
                    *color = Color::ORANGE_RED.into();
                }
            }
        }
    }
}

fn cleanup(mut commands: Commands, ui: Query<Entity, With<GameOverUI>>) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
