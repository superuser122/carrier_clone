use bevy::prelude::*;
use super::{game_grid::*, PlayerSpawned, player::Player};

#[derive(Component)]
pub struct GridEntity{
    pub cell_type: GridCellType,
}

#[derive(Component)]
pub struct ToMove(pub bool);

pub struct BlockNextMove(pub bool);

#[derive(Component)]
pub struct CurrentPosition(pub GridCell);

#[derive(Component, Debug)]
pub struct GridVelocity(pub GridCell);

// Newtype to use a `Timer` for this screen as a resource
#[derive(Deref, DerefMut)]
pub struct MoveTimer(pub Timer);

pub struct GridPhysicsPlugin;

impl Plugin for GridPhysicsPlugin{
    fn build(&self, app: &mut App){
        app
        .add_system_set(SystemSet::on_update(PlayerSpawned::Yes).with_system(move_movable))
        .add_system_set(SystemSet::on_update(PlayerSpawned::Yes).with_system(update_grid.after(move_movable)))
        .add_system_set(SystemSet::on_update(PlayerSpawned::Yes).with_system(ground_movable.after(update_grid)));
    }
}

pub fn update_grid(mut game_grid: ResMut<GameGrid>, query: Query<(&CurrentPosition, &GridEntity), With<ToMove>>){
    for (position, grid_entity) in query.iter(){
        let value = game_grid.grid.get_mut(&position.0).unwrap();
        *value = Some(grid_entity.cell_type);
    }
}

pub fn move_movable(
    mut query: Query<(&mut GridVelocity, &mut ToMove, &mut Transform, &mut CurrentPosition), Without<Player>>,
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut game_grid: ResMut<GameGrid>,
    mut block: ResMut<BlockNextMove>,
){
    timer.tick(time.delta());
    for (mut velocity, mut to_move, mut transform, mut position) in query.iter_mut() {

        if to_move.0{
            block.0 = true;
            transform.translation = position.0.as_vec3() + velocity.0.as_vec3() * timer.percent();
            if timer.finished() {
                to_move.0 = false;
                let value = game_grid.grid.get_mut(&position.0).unwrap();
                *value = None;
                position.0 = position.0 + velocity.0;
                velocity.0.reset();
                block.0 = false;
            }
        }
    };
}

fn ground_movable(
    mut query: Query<(&mut GridVelocity, &mut ToMove, &CurrentPosition, &GridEntity,)>,
    mut timer: ResMut<MoveTimer>,
    mut block: ResMut<BlockNextMove>,
    game_grid: Res<GameGrid>,
){
    for (mut velocity, mut to_move, position, grid_entity) in query.iter_mut(){
        let cell_below = position.0 + GridCell::new(0,-1,0);
        if cell_below.y < 0 { return;}
        if to_move.0 { return;}
        if game_grid.grid.get(&cell_below).unwrap().is_none() {
            to_move.0 = true;
            velocity.0 = GridCell::new(0,-1,0);
            timer.0.reset();
        }
    }

}
