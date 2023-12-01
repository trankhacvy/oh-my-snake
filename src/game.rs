use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, window::PrimaryWindow};
use rand::random;

use crate::GameState;

const SNAKE_HEAD_COLOR: Color = Color::WHITE;
const FOOD_COLOR: Color = Color::GREEN;
const SNAKE_BODY_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

pub struct GamePlayingPlugin;

impl Plugin for GamePlayingPlugin {
    fn build(&self, app: &mut App) {
        // resources
        app.insert_resource(SnakeBody::default())
            .insert_resource(TailPosition::default())
            .insert_resource(LastInputDirection(None))
            .insert_resource(ScoreBoard(0));

        // events
        app.add_event::<GrowthEvent>();
        app.add_event::<GameOverEvent>();

        // handler
        app.add_systems(OnEnter(GameState::Game), setup);

        app.add_systems(
            Update,
            (
                snake_movement.run_if(on_timer(Duration::from_millis(350))),
                spawn_food.run_if(on_timer(Duration::from_millis(700))),
                snake_movement_input.before(snake_movement),
                snake_eating.after(snake_movement),
                snake_growth.after(snake_eating),
                update_scoreboard.after(snake_eating),
                game_over.after(snake_movement),
            )
                .run_if(in_state(GameState::Game)),
        )
        // .add_systems(Update, update_scoreboard.run_if(in_state(GameState::Game)))
        .add_systems(PostUpdate, (size_scaling, position_translation));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct SnakeBody(Vec<Entity>);

#[derive(Component)]
struct SnakeBodyPart;

#[derive(Component)]
struct Food;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Event)]
struct GrowthEvent;

#[derive(Event)]
struct GameOverEvent;

#[derive(Resource, Default)]
struct TailPosition(Option<Position>);

#[derive(Resource)]
struct LastInputDirection(Option<Direction>);

#[derive(Resource)]
struct ScoreBoard(usize);

fn setup(mut commands: Commands, mut body: ResMut<SnakeBody>, asset_server: Res<AssetServer>) {
    // scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 24.0,
                    color: Color::YELLOW,
                    font: asset_server.load("fonts/KnightWarrior.otf"),
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 24.0,
                    color: Color::YELLOW,
                    font: asset_server.load("fonts/KnightWarrior.otf"),
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            margin: UiRect::axes(Val::Px(10.0), Val::Px(10.0)),
            ..default()
        }),
    );

    // snake

    let spawn_pos_x = ARENA_WIDTH as i32 / 2;
    let spawn_pos_y = ARENA_WIDTH as i32 / 2;

    *body = SnakeBody(vec![
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: SNAKE_HEAD_COLOR,
                        ..default()
                    },
                    ..default()
                },
                SnakeHead {
                    direction: Direction::Up,
                },
                SnakeBodyPart,
                Position {
                    x: spawn_pos_x,
                    y: spawn_pos_y,
                },
                Size::square(0.8),
            ))
            .id(),
        spawn_body(
            commands,
            Position {
                x: spawn_pos_x,
                y: spawn_pos_y - 1,
            },
        ),
    ]);
}

fn spawn_food(
    mut commands: Commands,
    body: ResMut<SnakeBody>,
    food_query: Query<Entity, With<Food>>,
    mut positions_query: Query<&mut Position>,
) {
    if food_query.iter().next().is_none() {
        let mut food_pos = Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        };

        let body_positions = body
            .0
            .iter()
            .map(|e| *positions_query.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        for body_pos in body_positions.iter() {
            if *body_pos == food_pos {
                food_pos = Position {
                    x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                    y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
                }
            }
        }

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: FOOD_COLOR,
                    ..default()
                },
                ..default()
            },
            Food,
            food_pos,
            Size::square(0.8),
        ));
    }
}

fn snake_movement_input(
    mut last_dir: ResMut<LastInputDirection>,
    keyboard_input: Res<Input<KeyCode>>,
    snake_query: Query<&SnakeHead>,
) {
    if let Ok(snake) = snake_query.get_single() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else {
            snake.direction
        };

        if dir != snake.direction.opposite() && dir != snake.direction && last_dir.0.is_none() {
            last_dir.0 = Some(dir);
        }
    }
}

fn snake_movement(
    body: ResMut<SnakeBody>,
    mut maybe_last_dir: ResMut<LastInputDirection>,
    mut tail_position: ResMut<TailPosition>,
    mut snake_head_query: Query<(Entity, &mut SnakeHead)>,
    mut positions_query: Query<&mut Position>,
    mut game_over_event: EventWriter<GameOverEvent>,
) {
    if let Ok((head_entity, mut head)) = snake_head_query.get_single_mut() {
        let body_positions = body
            .0
            .iter()
            .map(|e| *positions_query.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        let mut head_pos = positions_query.get_mut(head_entity).unwrap();

        if let Some(last_dir) = maybe_last_dir.0 {
            head.direction = last_dir;
            maybe_last_dir.0 = None;
        }

        match head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };

        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_event.send(GameOverEvent);
        }

        if body_positions.contains(&head_pos) {
            game_over_event.send(GameOverEvent);
        }

        body_positions
            .iter()
            .zip(body.0.iter().skip(1))
            .for_each(|(pos, part)| {
                *positions_query.get_mut(*part).unwrap() = *pos;
            });

        *tail_position = TailPosition(Some(*body_positions.last().unwrap()));
    }
}

fn size_scaling(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Size, &mut Transform)>,
) {
    let window = primary_query.get_single().unwrap();

    for (size, mut transform) in query.iter_mut() {
        transform.scale = Vec3::new(
            size.width / ARENA_WIDTH as f32 * window.width() as f32,
            size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - bound_window / 2.0 + tile_size / 2.0
    }

    let window = primary_query.get_single().unwrap();

    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            convert(position.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(
                position.y as f32,
                window.height() as f32,
                ARENA_HEIGHT as f32,
            ),
            0.0,
        );
    }
}

fn spawn_body(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_BODY_COLOR,
                    ..default()
                },
                ..default()
            },
            SnakeBodyPart,
            position,
            Size::square(0.6),
        ))
        .id()
}

fn snake_eating(
    mut commands: Commands,
    snake_query: Query<&Position, With<SnakeHead>>,
    food_query: Query<(Entity, &Position), With<Food>>,
    mut growth_ev_writer: EventWriter<GrowthEvent>,
) {
    for snake_pos in snake_query.iter() {
        for (food, food_pos) in food_query.iter() {
            if snake_pos == food_pos {
                commands.entity(food).despawn();
                growth_ev_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    commands: Commands,
    last_tail_pos: Res<TailPosition>,
    mut scoreboard: ResMut<ScoreBoard>,
    mut body: ResMut<SnakeBody>,
    mut growth_ev_reader: EventReader<GrowthEvent>,
) {
    if growth_ev_reader.read().next().is_some() {
        scoreboard.0 += 1;
        body.0.push(spawn_body(commands, last_tail_pos.0.unwrap()));
    }
}

fn game_over(
    mut commands: Commands,
    body: ResMut<SnakeBody>,
    food_query: Query<Entity, With<Food>>,
    body_parts_query: Query<Entity, With<SnakeBodyPart>>,
    mut game_over_ev_reader: EventReader<GameOverEvent>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game_over_ev_reader.read().next().is_some() {
        next_state.set(GameState::GameOver);

        for e in food_query.iter().chain(body_parts_query.iter()) {
            commands.entity(e).despawn();
        }

        setup(commands, body, asset_server);
    }
}

fn update_scoreboard(scoreboard: Res<ScoreBoard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.0.to_string();
}
