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
pub struct OnLevelNineScreen;

pub struct SceneNineHandle {
    pub handle: Handle<Gltf>,
    pub is_loaded: bool,
}

pub struct LevelNinePlugin;


impl Plugin for LevelNinePlugin{
    fn build(&self, app: &mut App){
        app
        // When entering the state, get the level from the info and set it
        .add_system_set(SystemSet::on_enter(GameLevel::LevelNine).with_system(load))
        .add_system_set(SystemSet::on_update(GameLevel::LevelNine).with_system(setup))
        .add_system_set(SystemSet::on_update(GameLevel::LevelNine).with_system(reset))
        .add_system_set(SystemSet::on_update(GameLevel::LevelNine).with_system(check_if_finished.after(setup)))
                .add_system_set(SystemSet::on_exit(GameLevel::LevelNine)
            .with_system(despawn_screen::<OnLevelNineScreen>),);
    }
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.insert_resource(SceneNineHandle {
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
    mut scene_handle: ResMut<SceneNineHandle>,
    mut next_move: ResMut<BlockNextMove>,
){

    if !scene_handle.is_loaded
    && asset_server.get_load_state(&scene_handle.handle) == LoadState::Loaded{
            scene_handle.is_loaded = true;
        let tiles0 = fs::read_to_string("assets/maps/LevelNine0.csv")
        .expect("Something went wrong reading the first file");

        let tiles1 = fs::read_to_string("assets/maps/LevelNine1.csv")
        .expect("Something went wrong reading the second file");

        let tiles = vec![tiles0,tiles1];
        fill_grid(&mut grid, tiles, 9,8,2);

        spawn_level(&mut commands, &mut meshes, &mut materials, OnLevelNineScreen, &mut grid);

        // camera
        commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(4.0, 10.0, 8.0).looking_at(Vec3::new(4.0, 0.0, 3.75), Vec3::Y),
            ..default()
        }).insert(OnLevelNineScreen);
        // light
        commands.spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 8.0, 0.0),
            ..default()
        }).insert(OnLevelNineScreen);


        //spawn reset button
        let _button = spawn_button(&mut commands, &asset_server, "Reset", OnLevelNineScreen);
        
        let player_position = GridCell::new(4,1,1);
        spawn_player(&mut commands, &assets_gltf, &glft_handle, player_position, OnLevelNineScreen);

        // block1
        let block1_position = GridCell::new(1,1,2);
        spawn_blocks(&mut commands, &glft_handle, block1_position, OnLevelNineScreen, 0);
        
        // block2
        let block2_position = GridCell::new(2,1,2);
        spawn_blocks(&mut commands, &glft_handle, block2_position, OnLevelNineScreen, 1);
        
        // block3
        let block3_position = GridCell::new(3,1,2);
        spawn_blocks(&mut commands, &glft_handle, block3_position, OnLevelNineScreen, 2);
        
        // block4
        let block4_position = GridCell::new(4,1,2);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelNineScreen, 3);

        // block5
        let block4_position = GridCell::new(5,1,2);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelNineScreen, 4);
        
        // block6
        let block4_position = GridCell::new(6,1,2);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelNineScreen, 5);

        // block7
        let block4_position = GridCell::new(7,1,2);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelNineScreen, 6);
        
        // block8
        let block4_position = GridCell::new(1,1,5);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelNineScreen, 7);
        
        // block9
        let block4_position = GridCell::new(2,1,5);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelNineScreen, 8);
        
        // block10
        let block4_position = GridCell::new(3,1,5);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelNineScreen, 9);
        
        // block11
        let block4_position = GridCell::new(5,1,5);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelNineScreen, 10);
        
        // block12
        let block4_position = GridCell::new(6,1,5);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelNineScreen, 11);

        // block13
        let block4_position = GridCell::new(7,1,5);
        spawn_blocks(&mut commands, &glft_handle, block4_position, OnLevelNineScreen, 12);
        
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(1.0, 0.51, 3.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(2.0, 0.51, 3.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(3.0, 0.51, 3.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(5.0, 0.51, 3.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(6.0, 0.51, 3.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(7.0, 0.51, 3.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(1.0, 0.51, 4.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(2.0, 0.51, 4.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(3.0, 0.51, 4.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(4.0, 0.51, 4.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(5.0, 0.51, 4.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(6.0, 0.51, 4.0),
            ..default()
        }).insert(OnLevelNineScreen);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.20 })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(7.0, 0.51, 4.0),
            ..default()
        }).insert(OnLevelNineScreen);

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
                level_reset.level = GameLevel::LevelNine;
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
    let target_cell1 = GridCell::new(1,1,3);
    let target_cell2 = GridCell::new(2,1,3);
    let target_cell3 = GridCell::new(3,1,3);
    let target_cell4 = GridCell::new(5,1,3);
    let target_cell5 = GridCell::new(6,1,3);
    let target_cell6 = GridCell::new(7,1,3);
    let target_cell7 = GridCell::new(1,1,5);
    let target_cell8 = GridCell::new(2,1,5);
    let target_cell9 = GridCell::new(3,1,5);
    let target_cell10 = GridCell::new(4,1,5);
    let target_cell11 = GridCell::new(5,1,5);
    let target_cell12 = GridCell::new(6,1,5);
    let target_cell13 = GridCell::new(7,1,5);

    let target1 = grid.grid.get(&target_cell1).unwrap();
    let target2 = grid.grid.get(&target_cell2).unwrap();
    let target3 = grid.grid.get(&target_cell3).unwrap();
    let target4 = grid.grid.get(&target_cell4).unwrap();
    let target5 = grid.grid.get(&target_cell5).unwrap();
    let target6 = grid.grid.get(&target_cell6).unwrap();
    let target7 = grid.grid.get(&target_cell7).unwrap();
    let target8 = grid.grid.get(&target_cell8).unwrap();
    let target9 = grid.grid.get(&target_cell9).unwrap();
    let target10 = grid.grid.get(&target_cell10).unwrap();
    let target11 = grid.grid.get(&target_cell11).unwrap();
    let target12 = grid.grid.get(&target_cell12).unwrap();
    let target13 = grid.grid.get(&target_cell13).unwrap();

    let cell_types = vec![target1, target2, target3,  target4, target5, target6,
     target7, target8, target9,  target10, target11, target12, target13];
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
            level.set(GameLevel::LevelNine).unwrap();
        }


    }
}


