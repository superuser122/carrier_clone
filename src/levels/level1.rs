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
pub struct OnLevelOneScreen;

pub struct LevelOnePlugin;

pub struct SceneOneHandle {
    pub handle: Handle<Gltf>,
    pub is_loaded: bool,
}

impl Plugin for LevelOnePlugin{
    fn build(&self, app: &mut App){
        app
        // When entering the state, get the level from the info and set it
        .add_system_set(SystemSet::on_enter(GameLevel::LevelOne).with_system(load))
        .add_system_set(SystemSet::on_update(GameLevel::LevelOne).with_system(setup))
        .add_system_set(SystemSet::on_update(GameLevel::LevelOne).with_system(reset))
        .add_system_set(SystemSet::on_update(GameLevel::LevelOne).with_system(check_if_finished.after(setup)))
                .add_system_set(SystemSet::on_exit(GameLevel::LevelOne)
            .with_system(despawn_screen::<OnLevelOneScreen>),);
        
        
    }
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.insert_resource(SceneOneHandle {
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
    mut scene_handle: ResMut<SceneOneHandle>,
    mut next_move: ResMut<BlockNextMove>,
){

    if !scene_handle.is_loaded
    && asset_server.get_load_state(&scene_handle.handle) == LoadState::Loaded{
            scene_handle.is_loaded = true;
        let tiles0 = fs::read_to_string("assets/maps/levelone0.csv")
        .expect("Something went wrong reading the first file");

        let tiles1 = fs::read_to_string("assets/maps/levelone1.csv")
        .expect("Something went wrong reading the second file");

        let tiles = vec![tiles0,tiles1];
        fill_grid(&mut grid, tiles, 8,9,2);

        spawn_level(&mut commands, &mut meshes, &mut materials, OnLevelOneScreen, &mut grid);

        // camera
        commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(3.75, 10.0, 10.0).looking_at(Vec3::new(3.75, 0.0, 4.0), Vec3::Y),
            ..default()
        }).insert(OnLevelOneScreen);
        // light
        commands.spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 8.0, 0.0),
            ..default()
        }).insert(OnLevelOneScreen);

        // commands.insert_resource(AmbientLight {
        //     color: Color::rgb_u8(210, 220, 240),
        //     brightness: 0.3,
        // });
        
        //spawn reset button
        let mut button = spawn_button(&mut commands, &asset_server, "Reset", OnLevelOneScreen);

        commands.entity(button).insert(ResetButton);


        let player_position = GridCell::new(6,1,4);
        spawn_player(&mut commands, &assets_gltf, &glft_handle, player_position, OnLevelOneScreen);

        // block1
        let block1_position = GridCell::new(2,1,2);
        spawn_blocks(&mut commands, &glft_handle, block1_position, OnLevelOneScreen, 0);
        
        // block2
        let block2_position = GridCell::new(4,1,4);
        spawn_blocks(&mut commands, &glft_handle, block2_position, OnLevelOneScreen, 1);
        
        // block3
        let block3_position = GridCell::new(2,1,6);
        spawn_blocks(&mut commands, &glft_handle, block3_position, OnLevelOneScreen, 2);
      
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(6.0, 0.76, 5.0),
            ..default()
        }).insert(OnLevelOneScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(6.0, 0.76, 6.0),
            ..default()
        }).insert(OnLevelOneScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(6.0, 0.76, 7.0),
            ..default()
        }).insert(OnLevelOneScreen);
        
        next_move.0 = false;
        player_spawned.replace(PlayerSpawned::Yes).unwrap();
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
                level_reset.level = GameLevel::LevelOne;
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
    let target_cell1 = GridCell::new(6,1,5);
    let target_cell2 = GridCell::new(6,1,6);
    let target_cell3 = GridCell::new(6,1,7);

    let target1 = grid.grid.get(&target_cell1).unwrap();
    let target2 = grid.grid.get(&target_cell2).unwrap();
    let target3 = grid.grid.get(&target_cell3).unwrap();

    let cell_types = vec![target1, target2, target3];
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
            player_spawned.replace(PlayerSpawned::No).expect("Something went wrong setting player state to no");
            level.replace(GameLevel::LevelTwo).expect("Something went wrong setting level state to two");
            timer.0.reset();
        }


    }
}


