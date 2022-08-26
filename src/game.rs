use bevy:: prelude::*;
use super::{GameState, GameLevel};

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app
        // When entering the state, get the level from the info and set it
        .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup));
    }
}

fn game_setup(
    mut level_state: ResMut<State<GameLevel>>,
){
    level_state.replace(GameLevel::LevelOne).expect("The game level state wasn't setted successfully");
}