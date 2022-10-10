use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use bevy::{prelude::*, gltf::Gltf};
use super::{grid_physics::*, PlayerSpawned, despawn_screen, game_grid::*,};
use std::f32::consts::TAU;
use super::game::{PlayerRotationTimer, GltfHandle};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, FromPrimitive)]
pub enum Facing {
    Up,
    Down,
    Right,
    Left,
    
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, FromPrimitive)]
pub enum PlayerState{
    Idle,
    Walking,
}

#[derive(Component)]
pub struct Player{
    pub current_facing: Facing,
    pub target_facing: Facing,
    pub state: PlayerState,
}

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerRotation(pub bool);

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
        .add_system_set(SystemSet::on_update(PlayerSpawned::Yes).with_system(player_input.after(update_grid)))
        .add_system_set(SystemSet::on_update(PlayerSpawned::Yes).with_system(move_player.after(player_input)))
        .add_system_set(SystemSet::on_update(PlayerSpawned::Yes).with_system(rotate_player))
//        .add_system_set(SystemSet::on_update(PlayerSpawned::Yes).with_system(animate_player))
        .add_system_set(SystemSet::on_exit(PlayerSpawned::Yes).with_system(despawn_screen::<Player>));
        
    }
}


pub fn spawn_player(commands: &mut Commands,
    assets_gltf: &Res<Assets<Gltf>>,
    glft_handle: &ResMut<GltfHandle>,
    position: GridCell,
    screen: impl Component + Clone,
){
    let gl =  assets_gltf.get(&glft_handle.gltf_handle).unwrap();
    let player_scene = gl.scenes[0].clone();
     commands.spawn()
     .insert(ComputedVisibility::default())
     .insert(Visibility{ is_visible: true})
     .insert(Transform {
         translation: position.clone().as_vec3(),
         ..Default::default()
     })
     .insert(GlobalTransform::default())
     .insert(Player{ current_facing:Facing::Down, target_facing: Facing::Down, state: PlayerState::Idle})
     .insert(PlayerRotation(false))
     .insert(GridEntity{ cell_type: GridCellType::Player})
     .insert(CurrentPosition(position.clone()))
     .insert(GridVelocity(GridCell::new(0,0,0)))
     .insert(ToMove(false))
     .insert(screen.clone())
     .with_children( |parent|{
         parent.spawn_bundle(SceneBundle {
             scene: player_scene,
             transform: Transform {
                 translation: Vec3::new(0.0, -0.5, 0.0),
                 scale: Vec3::new(0.7, 0.7, 0.7),
                 ..Default::default()
             },
             ..default()
         }).insert(screen.clone());
     });

}

fn player_input(
    mut player_query: Query<(&mut CurrentPosition, &mut GridVelocity, &mut ToMove, &mut PlayerRotation, &mut Player), With<Player>>,
    mut tile_query: Query<(&mut CurrentPosition, &mut GridVelocity, &mut ToMove, &GridEntity), Without<Player>>,
    input: Res<Input<KeyCode>>, 
    game_grid: Res<GameGrid>,
    mut timer: ResMut<MoveTimer>,
    block: Res<BlockNextMove>,
    mut rotation_timer: ResMut<PlayerRotationTimer>

){
    if block.0 { return;}
    for (player_position, mut player_velocity, mut move_player, mut player_rotation, mut player) in player_query.iter_mut(){

        if move_player.0 {return;}
        let mut check_input = false;
        if input.just_pressed(KeyCode::Left){
            player_velocity.0.x = -1;
            check_input = true;
            
            rotation_timer.0.reset();
                player.target_facing = Facing::Left;
            
        }
        else if input.just_pressed(KeyCode::Right){
            player_velocity.0.x = 1;
            check_input = true;
           
            rotation_timer.0.reset();
                player.target_facing = Facing::Right;
            
        }
        else if input.just_pressed(KeyCode::Up){
            player_velocity.0.z = -1;
            check_input = true;
            
            rotation_timer.0.reset();
                player.target_facing = Facing::Up;
        }
        else if input.just_pressed(KeyCode::Down){
            player_velocity.0.z = 1;
            check_input = true;
            
            rotation_timer.0.reset();
                player.target_facing = Facing::Down;
            
        }
        

        player_rotation.0 =  input.just_pressed(KeyCode::R);

        if !check_input {return;}
        
        let target_position = player_position.0 + player_velocity.0;
        let player_target_cell = game_grid.grid.get(&target_position).unwrap();
        match player_target_cell {
            None => {
                let one_cell_below = target_position + GridCell::new(0,-1,0);
                let ground_gap = game_grid.grid.get(&one_cell_below).unwrap();
                if ground_gap.is_none() {
                    player_velocity.0.reset();
                    return;
                };
                move_player.0 = true;
                timer.0.reset();
            },
            Some(cell) => {
                match cell {
                    GridCellType::MovingTile(id) =>{
                        for (tile_position,
                            mut tile_velocity,
                            mut move_tile,
                            grid_entity ) 
                            in tile_query.iter_mut()
                        {
                            if grid_entity.cell_type != GridCellType::MovingTile(id.clone()) { continue; } 

                            let tile_target_position = tile_position.0 + player_velocity.0;
                            let tile_target_cell = game_grid.grid.get(&tile_target_position).unwrap();
                            if tile_target_cell.is_none(){
                                move_player.0 = true;
                                move_tile.0 = true;
                                tile_velocity.0 = player_velocity.0;
                                timer.0.reset();
                            }else{
                                player_velocity.0.reset();
                            }
                        }
                    },
                    GridCellType::Tile(_) => player_velocity.0.reset(),
                    _ => {}
                }
            }, 
        }
    }
}


fn move_player(
    mut query: Query<(&mut GridVelocity, &mut ToMove, &mut Transform, &mut CurrentPosition), With<Player>>,
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut game_grid: ResMut<GameGrid>
){
    timer.tick(time.delta());
    for (mut velocity, mut to_move, mut transform, mut position) in query.iter_mut() {

        if to_move.0{
            transform.translation = position.0.as_vec3() + velocity.0.as_vec3() * timer.percent();
            if timer.finished() {
                to_move.0 = false;
                let value = game_grid.grid.get_mut(&position.0).unwrap();
                *value = None;
                position.0 = position.0 + velocity.0;
                velocity.0.reset();
            }
        }
    };
 
}

// This system will rotate any entity in the scene with a Rotatable component around its y-axis.
fn rotate_player(mut query: Query<(&mut Transform, &mut Player), With<Player>>, time: Res<Time>, mut timer: ResMut<PlayerRotationTimer>) {
    let (mut transform, mut player) = query.single_mut(); 
    timer.0.tick(time.delta());
    if !timer.0.finished(){
         let  rotation:f32 =  match player.target_facing {
            Facing::Up => {
                std::f32::consts::PI
            },
            Facing::Down => {
                0.0
            },
            Facing::Right => {
                std::f32::consts::FRAC_PI_2
            },
            Facing::Left => {
                -std::f32::consts::FRAC_PI_2
            },
        };
        let q = Quat::from_rotation_y(rotation);
        transform.rotation = transform.rotation.lerp(q, timer.0.percent());
        return;
    }
    player.current_facing = player.target_facing;

}

fn animate_player(
    mut anim_player: Query<&mut AnimationPlayer>,
     glft_handle: ResMut<GltfHandle>,
     query: Query<&ToMove, With<Player>>,
     assets_gltf: Res<Assets<Gltf>>,
    ){
        let gl =  assets_gltf.get(&glft_handle.gltf_handle).unwrap();
    if let Ok(mut player) = anim_player.get_single_mut() {
        let to_move = query.single();
        if to_move.0 {
            player.play(gl.animations[1].clone_weak()).repeat();
            return;
        }
        player.play(gl.animations[2].clone_weak()).repeat();
        
    }
}