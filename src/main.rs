use bevy::prelude::*;
mod splash;
use splash::SplashPlugin;
mod ui;
mod menu;
use menu::MenuPlugin;
mod pause;
use pause::GamePausePlugin;
mod game;
use game::GamePlugin;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Load,
    Menu,
    Game,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameLevel{
    None,
    Pause,
    LevelOne,
    LevelTwo,
    LevelThree,
    LevelFour,
    LevelFive,
    LevelSix,
    LevelSeven,
    LevelEight,
    LevelNine,
    LevelTen,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerSpawned{
    Yes,
    No,
    Paused,
}

fn main() {
    let window = WindowDescriptor {
        title: "Carrier Clone".to_string(),
        resizable: false,
        ..Default::default()
    };
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(window)
        .add_plugins(DefaultPlugins)
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePausePlugin)
        .add_plugin(GamePlugin)
        .add_state(GameState::Splash)
        .add_state(PlayerSpawned::No)
        .add_state(GameLevel::None)
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

