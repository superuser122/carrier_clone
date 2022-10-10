use bevy::prelude::*;
use super::game_grid::{GameGrid, GridCell, GridCellType};
use super::grid_physics::{GridEntity, CurrentPosition, GridVelocity, ToMove,};
use super::game::GltfHandle;


pub fn spawn_blocks(
    commands: &mut Commands,
    gltf_handle:  &ResMut<GltfHandle>,
    position: GridCell,
    screen: impl Component + Clone,
    tile_id: i32
){
    commands.spawn_bundle(SceneBundle {
        scene: gltf_handle.box_handle.clone(),
        transform: Transform {
            translation: position.clone().as_vec3(),
            ..Default::default()
        },
        ..default()
    })
    .insert(GridEntity{ cell_type: GridCellType::MovingTile(tile_id)})
    .insert(CurrentPosition(position.clone()))
    .insert(GridVelocity(GridCell::new(0,0,0)))
    .insert(ToMove(false))
    .insert(screen.clone());
}

pub fn fill_grid(grid:&mut ResMut<GameGrid>, tiles: Vec<String>, width: i32, length: i32, heigth: i32){
    grid.reset(width, length, heigth);
    for (y, csv) in tiles.iter().enumerate(){
        for(z, line) in csv.lines().enumerate(){
            let indexes = line.split(",").collect::<Vec<&str>>();
            for(x, str_index) in indexes.into_iter().enumerate(){
                let index = str_index.parse::<i32>().unwrap();
                if index < 0 { continue; }
                let cell = GridCell{x: x as i32, y: y as i32, z: z as i32};
                let value = grid.grid.get_mut(&cell).unwrap();
                *value = Some(GridCellType::Tile(index));
            }
        }
    }
}

pub fn spawn_level(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    screen: impl Component + Clone,
    grid: &mut ResMut<GameGrid>,
){
    let mesh_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let material_handle0 = materials.add(Color::rgb(0.3, 0.5, 0.3).into());
    let material_handle1 = materials.add(Color::rgb(0.7, 0.7, 0.3).into());

    for (k, v) in grid.grid.iter(){
        if let Some(tile) = *v{
            match tile {
                GridCellType::Tile(id) => {
                    match id {
                        0 => {
                            commands.spawn_bundle(PbrBundle {
                                mesh: mesh_handle.clone(),
                                material: material_handle0.clone(),
                                transform: Transform::from_xyz(k.x as f32, k.y as f32, k.z as f32),
                                ..default()
                            })
                            .insert(GridEntity{ cell_type: GridCellType::Tile(0)})
                            .insert(screen.clone())
                            .insert(CurrentPosition(GridCell::new(k.x, k.y, k.z)));
                        },
                        1 =>{
                            commands.spawn_bundle(PbrBundle {
                                mesh: mesh_handle.clone(),
                                material: material_handle1.clone(),
                                transform: Transform::from_xyz(k.x as f32, k.y as f32, k.z as f32),
                                ..default()
                            })
                            .insert(GridEntity{ cell_type: GridCellType::Tile(1)})
                            .insert(screen.clone())
                            .insert(CurrentPosition(GridCell::new(k.x, k.y, k.z)));

                        },
                        _ => {},
                    }

                }
                _ => {}
            }

        }
    }

}