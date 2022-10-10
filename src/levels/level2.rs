use bevy::{prelude::*, asset::LoadState, gltf::{Gltf, GltfMesh},};
use crate::PlayerSpawned;
use crate::game_grid::*;
use crate::grid_physics::*;
use std::fs;
use crate::player::{Player, PlayerRotation, Facing, spawn_player};
use super::GameLevel;
use super::game::{LevelReset, WinTimer, GltfHandle};
use super::despawn_screen;
use super::level_entities::{spawn_blocks, fill_grid, spawn_level};
use super::ui::*;


#[derive(Component, Clone, Copy)]
pub struct OnLevelTwoScreen;

pub struct SceneTwoHandle {
    pub handle: Handle<Gltf>,
    pub is_loaded: bool,
}

pub struct LevelTwoPlugin;


impl Plugin for LevelTwoPlugin{
    fn build(&self, app: &mut App){
        app
        // When entering the state, get the level from the info and set it
        .add_system_set(SystemSet::on_enter(GameLevel::LevelTwo).with_system(load))
        .add_system_set(SystemSet::on_update(GameLevel::LevelTwo).with_system(setup))
        .add_system_set(SystemSet::on_update(GameLevel::LevelTwo).with_system(reset))
        .add_system_set(SystemSet::on_update(GameLevel::LevelTwo).with_system(check_if_finished.after(setup)))
                .add_system_set(SystemSet::on_exit(GameLevel::LevelTwo)
            .with_system(despawn_screen::<OnLevelTwoScreen>),);
        
        
    }
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.insert_resource(SceneTwoHandle {
        handle: asset_server.load("warrior_02.glb"),
        is_loaded: false,
    });

}


fn setup(
    mut commands: Commands,
    mut grid: ResMut<GameGrid>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets_gltf: Res<Assets<Gltf>>,
    glft_handle: ResMut<GltfHandle>,
    asset_server: Res<AssetServer>,
    mut player_spawned: ResMut<State<PlayerSpawned>>,
    mut scene_handle: ResMut<SceneTwoHandle>,
    mut next_move: ResMut<BlockNextMove>,
){

    if !scene_handle.is_loaded
    && asset_server.get_load_state(&scene_handle.handle) == LoadState::Loaded{
            scene_handle.is_loaded = true;
        let tiles0 = fs::read_to_string("assets/maps/leveltwo0.csv")
        .expect("Something went wrong reading the first file");

        let tiles1 = fs::read_to_string("assets/maps/leveltwo1.csv")
        .expect("Something went wrong reading the second file");

        let tiles = vec![tiles0,tiles1];
        fill_grid(&mut grid, tiles, 8,6,2);

        spawn_level(&mut commands, &mut meshes, &mut materials, OnLevelTwoScreen, &mut grid);

        // camera
        commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(3.25, 9.0, 8.0).looking_at(Vec3::new(3.25, 0.0, 3.0), Vec3::Y),
            ..default()
        }).insert(OnLevelTwoScreen);
        // light
        commands.spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 8.0, 0.0),
            ..default()
        }).insert(OnLevelTwoScreen);


        //spawn reset button
        let _button = spawn_button(&mut commands, &asset_server, "Reset", OnLevelTwoScreen);

        let player_position = GridCell::new(5,1,1);
        spawn_player(&mut commands, &assets_gltf, &glft_handle, player_position, OnLevelTwoScreen);

        // block1
        let block1_position = GridCell::new(5,1,2);
        spawn_blocks(&mut commands,&glft_handle, block1_position, OnLevelTwoScreen, 0);
        
        // block2
        let block2_position = GridCell::new(4,1,2);
        spawn_blocks(&mut commands, &glft_handle, block2_position, OnLevelTwoScreen, 1);
        
        // block3
        let block3_position = GridCell::new(3,1,3);
        spawn_blocks(&mut commands, &glft_handle, block3_position, OnLevelTwoScreen, 2);
        
        // block4
        let block4_position = GridCell::new(2,1,2);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelTwoScreen, 3);
        
        // block4
        let block5_position = GridCell::new(1,1,3);
        spawn_blocks(&mut commands, &glft_handle, block5_position, OnLevelTwoScreen, 4);

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(1.0, 0.51, 1.0),
            ..default()
        }).insert(OnLevelTwoScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(2.0, 0.51, 1.0),
            ..default()
        }).insert(OnLevelTwoScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(1.0, 0.51, 2.0),
            ..default()
        }).insert(OnLevelTwoScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(1.0, 0.51, 3.0),
            ..default()
        }).insert(OnLevelTwoScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(1.0, 0.51, 4.0),
            ..default()
        }).insert(OnLevelTwoScreen);
        
        // commands.insert_resource(MoveTimer(Timer::from_seconds(0.3, false)));
        // commands.insert_resource(BlockNextMove(false));
        next_move.0 = false;
        player_spawned.replace(PlayerSpawned::Yes).expect("Something went wrong setting player state to no");
    }


    
}

fn reset(
    mut level_reset: ResMut<LevelReset>,
    mut level_state: ResMut<State<GameLevel>>,
    mut player_spawned: ResMut<State<PlayerSpawned>>,
    mut interaction_query: Query<
    (&Interaction,&mut Transform),
    (Changed<Interaction>, With<Button>, Without<ToMove>, Without<Player>)>
){
    for (interaction, mut button_transform)  in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                player_spawned.replace(PlayerSpawned::No).expect("Player pop state wasn't successful");
                level_reset.reset = true;
                level_reset.level = GameLevel::LevelTwo;
                level_state.replace(GameLevel::None).expect("The game level state wasn't setted successfully");

            },
            Interaction::Hovered => {
                button_transform.scale = Vec3::new(1.2, 1.2, 1.);
            },
            Interaction::None => {
                button_transform.scale = Vec3::new(1., 1., 1.);
            },
        }
    }
}

fn check_if_finished(
    grid: Res<GameGrid>,
    mut player_spawned: ResMut<State<PlayerSpawned>>,
    mut block: ResMut<BlockNextMove>,
    mut level: ResMut<State<GameLevel>>,
    mut timer: ResMut<WinTimer>,
    time: Res<Time>,
){
    if player_spawned.current() != &PlayerSpawned::Yes { return;}
    let target_cell1 = GridCell::new(1,1,1);
    let target_cell2 = GridCell::new(2,1,1);
    let target_cell3 = GridCell::new(1,1,2);
    let target_cell4 = GridCell::new(1,1,3);
    let target_cell5 = GridCell::new(1,1,4);

    let target1 = grid.grid.get(&target_cell1).unwrap();
    let target2 = grid.grid.get(&target_cell2).unwrap();
    let target3 = grid.grid.get(&target_cell3).unwrap();
    let target4 = grid.grid.get(&target_cell4).unwrap();
    let target5 = grid.grid.get(&target_cell5).unwrap();

    let cell_types = vec![target1, target2, target3,  target4, target5];
    let mut no_match: bool = false;
    for cell in cell_types.iter(){
        match cell {
            Some(cell_type) => {
                match cell_type {
                    GridCellType::MovingTile(_) => {},
                    _ => { no_match = true;}
                }
            },
            None => {
                no_match = true;
            },
        } 
    }
    if !no_match{
        block.0 = true;
        timer.0.tick(time.delta());
        if timer.0.finished(){
            player_spawned.replace(PlayerSpawned::No).unwrap();
            level.set(GameLevel::LevelThree).unwrap();
        }


    }
}

