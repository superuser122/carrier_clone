use bevy::{prelude::*, gltf::Gltf};
use super::{GameState, GameLevel, game_grid::GameGrid,};
use super::grid_physics::{BlockNextMove, MoveTimer};
pub struct GamePlugin;

pub struct GltfHandle {
    pub gltf_handle: Handle<Gltf>,
    pub box_handle: Handle<Scene>
}

pub struct LevelReset{
    pub level: GameLevel,
    pub reset: bool,
}

pub struct WinTimer(pub Timer);

pub struct PlayerRotationTimer(pub Timer);

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app
        // When entering the state, get the level from the info and set it
        .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup))
        .add_system_set(SystemSet::on_update(GameState::Game).with_system(reset));
    }
}




fn setup(
    mut level_state: ResMut<State<GameLevel>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.insert_resource(GltfHandle {
        gltf_handle: asset_server.load("warrior_01.glb"),
        box_handle: asset_server.load("box.glb#Scene0"),
    });

    let game_map = GameGrid::new(1, 1, 1);
    commands.insert_resource(WinTimer(Timer::from_seconds(2.0, false)));
    commands.insert_resource(PlayerRotationTimer(Timer::from_seconds(0.3, false)));
    commands.insert_resource(game_map);
    commands.insert_resource(BlockNextMove(false));
    commands.insert_resource(MoveTimer(Timer::from_seconds(0.3, false)));
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.3,
    });
    commands.insert_resource(LevelReset{
        level: GameLevel::None,
        reset: false,
    });

    
    level_state.replace(GameLevel::LevelOne).expect("The game level state wasn't setted successfully");


    

}

fn reset(mut level_reset: ResMut<LevelReset>,mut level_state: ResMut<State<GameLevel>>, ){
    if !level_reset.reset {return;}
    if level_state.current() != &GameLevel::None {return;}
    //println!("{:?}", level_state);
    level_state.replace(level_reset.level).expect("The game level state wasn't reseted successfully");
    level_reset.reset = false;

}