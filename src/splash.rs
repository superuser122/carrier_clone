use bevy::prelude::*;
use super::{GameState, despawn_screen};


// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Deref, DerefMut)]
struct LogoTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct FadeEffectTimer(Timer);

#[derive(Component)]
pub struct LogoNum(u8);

pub struct SplashPlugin;

impl Plugin for SplashPlugin{
    fn build(&self, app: &mut App){
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
        // When entering the state, spawn everything needed for this screen
        .add_system_set(SystemSet::on_enter(GameState::Splash).with_system(setup))
        // While in this state, run the `countdown` system
        .add_system_set(SystemSet::on_update(GameState::Splash).with_system(countdown))
        // When exiting the state, despawn everything that was spawned for this screen
        .add_system_set(
            SystemSet::on_exit(GameState::Splash)
                .with_system(despawn_screen::<OnSplashScreen>),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bevy_logo.png"),
        ..default()
    })
    .insert(OnSplashScreen)
    .insert(LogoNum(0))
    .insert(FadeEffectTimer(Timer::from_seconds(2.0, false)));

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("shiro_logo.png"),
        visibility : Visibility{ is_visible: false},
        ..default()
    })
    .insert(OnSplashScreen)
    .insert(LogoNum(1))
    .insert(FadeEffectTimer(Timer::from_seconds(2.0, false)));
    // Insert the timer as a resource
    commands.insert_resource(LogoTimer(Timer::from_seconds(4.0, false)));

}

// Tick the timer, and change state when finished
fn countdown(mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<LogoTimer>,
    mut query: Query<(&mut Visibility,&mut FadeEffectTimer, &mut Sprite,  &LogoNum)>,
    ) {
  
  timer.tick(time.delta());
  
  for (mut visibility, mut fade_timer,mut sprite, logo_num) in query.iter_mut(){
      //This part of code has doublicates and needs refuctoring 
      if logo_num.0 == 0 {
          if timer.percent() < 0.5{
              visibility.is_visible = true;
              fade_in_out(&mut fade_timer.0, &time, sprite.as_mut());
          }
          else{
              visibility.is_visible = false;
          }

      }
      if  logo_num.0 == 1{
          if timer.percent() > 0.5{
              visibility.is_visible = true;
              fade_in_out(&mut fade_timer.0, &time, sprite.as_mut());
          }
          else{
              visibility.is_visible = false;
          }

      }
  }
  
  if timer.finished() {
      game_state.set(GameState::Menu).unwrap();
  }
}

pub fn fade_in_out(timer: &mut Timer, time: &Res<Time>, sprite: &mut Sprite){
    timer.tick(time.delta());
    if !timer.finished(){
        let alpha = if timer.percent() < 0.3 {
            timer.percent() * 2.0
        } 
        else if timer.percent() > 0.3 && timer.percent() < 0.7{
            1.
        }
        else {
            timer.percent_left() * 2.0
        };
        sprite.color.set_a(alpha);
    }
}