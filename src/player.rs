use bevy::prelude::*;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, create_animations)
        .add_system_set(SystemSet::on_update(PlayerSpawned::Yes).with_system(player_movement))
        .add_system_set(SystemSet::on_exit(PlayerSpawned::Yes).with_system(despawn_screen::<Player>));
        
    }
}