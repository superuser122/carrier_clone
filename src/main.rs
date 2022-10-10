use bevy::{prelude::*, gltf::Gltf};
mod splash;
use splash::SplashPlugin;
mod ui;
mod menu;
use menu::MenuPlugin;
mod pause;
use pause::GamePausePlugin;
mod game;
use game::GamePlugin;
mod game_grid;
mod grid_physics;
use grid_physics::GridPhysicsPlugin;
mod player;
use player::PlayerPlugin;
mod levels;
use levels::{
    level1::LevelOnePlugin,
    level2::LevelTwoPlugin,
    level3::LevelThreePlugin,
    level4::LevelFourPlugin,
    level5::LevelFivePlugin,
    level6::LevelSixPlugin,
    level7::LevelSevenPlugin,
    level8::LevelEightPlugin,
    level9::LevelNinePlugin,
    level10::LevelTenPlugin,
 };
mod level_entities;
use level_entities::{spawn_blocks,};


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Load,
    Menu,
    Game,
    Credits,
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


pub struct Scale(f32);


fn main() {
    let window = WindowDescriptor {
        title: "Carrier Clone".to_string(),
        width: 1920.0,
        height: 1080.0,
        scale_factor_override: Some(0.8),

        //resizable: false,
        ..Default::default()
    };
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(window)
        .insert_resource(Scale(1.0))
        .add_plugins(DefaultPlugins)
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePausePlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GridPhysicsPlugin)
        .add_plugin(LevelOnePlugin)
        .add_plugin(LevelTwoPlugin)
        .add_plugin(LevelThreePlugin)
        .add_plugin(LevelFourPlugin)
        .add_plugin(LevelFivePlugin)
        .add_plugin(LevelSixPlugin)
        .add_plugin(LevelSevenPlugin)
        .add_plugin(LevelEightPlugin)
        .add_plugin(LevelNinePlugin)
        .add_plugin(LevelTenPlugin)
        .add_plugin(PlayerPlugin)
        .add_state(GameState::Splash)
        .add_state(PlayerSpawned::No)
        .add_state(GameLevel::None)
        .run();
}

fn reset_scale(mut windows: ResMut<Windows>,){
    let window = windows.primary_mut();
    let win_widht = window.width();
    let scale = (win_widht/1920.0) as f64;
    window.set_scale_factor_override(Some(scale));
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

