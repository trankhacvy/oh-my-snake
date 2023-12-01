use bevy::prelude::*;
use game::GamePlayingPlugin;
use game_over::GameOverPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;

mod game;
mod game_over;
mod loading;
mod menu;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Game,
    Menu,
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            GamePlayingPlugin,
            GameOverPlugin,
        ));
    }
}
