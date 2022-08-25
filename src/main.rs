use bevy::prelude::*;
mod splash;
use splash::SplashPlugin;
mod ui;
use ui::{spawn_button, spawn_menu_container};
mod menu;
use menu::MenuPlugin;


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
    Level
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
        .add_state(GameState::Splash)
        .add_state(GameLevel::None)
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

