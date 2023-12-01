use bevy::{app::AppExit, prelude::*};

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu);
        app.add_systems(
            Update,
            handle_buttons_interaction.run_if(in_state(GameState::Menu)),
        );
        app.add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct Play;

#[derive(Component)]
struct Quit;

#[derive(Component)]
struct OpenLink(&'static str);

const GITHUB_URL: &str = "https://github.com/trankhacvy/oh-my-snake";

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK),
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            // text
            children.spawn(
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "_Snake",
                    TextStyle {
                        font: asset_server.load("fonts/KnightWarrior.otf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Center),
            );

            // play button
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
                    Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/KnightWarrior.otf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // quit button
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
                        background_color: BackgroundColor::from(Color::ORANGE_RED),
                        ..Default::default()
                    },
                    Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: asset_server.load("fonts/KnightWarrior.otf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::End,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    padding: UiRect::horizontal(Val::Px(40.)),
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    OpenLink(GITHUB_URL),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Source code",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            font: asset_server.load("fonts/KnightWarrior.otf"),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: UiImage::new(asset_server.load("textures/github.png")),
                        style: Style {
                            width: Val::Px(24.),
                            ..default()
                        },
                        ..default()
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
            Option<&Play>,
            Option<&Quit>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, maybe_btn_play, maybe_btn_quit) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(_) = maybe_btn_play {
                    next_state.set(GameState::Game);
                } else if let Some(_) = maybe_btn_quit {
                    app_exit_events.send(AppExit);
                }
            }
            Interaction::Hovered => {
                if let Some(_) = maybe_btn_play {
                    *color = Color::rgba(0.93, 0.51, 0.93, 0.5).into();
                } else if let Some(_) = maybe_btn_quit {
                    *color = Color::rgba(1.0, 0.27, 0.0, 0.5).into();
                }
            }
            Interaction::None => {
                if let Some(_) = maybe_btn_play {
                    *color = Color::VIOLET.into();
                } else if let Some(_) = maybe_btn_quit {
                    *color = Color::ORANGE_RED.into();
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
