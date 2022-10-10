use bevy::{prelude::*, app::AppExit, render::view::visibility};
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use super::{despawn_screen, GameLevel, GameState, PlayerSpawned, ui::*};

pub struct GamePausePlugin;

//Helps to despawn entities
#[derive(Component)]
struct OnPauseScreen;

//Enum values representing all the buttons
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, FromPrimitive)]
pub enum PauseButtonType{
    ResumeGame,
    Quit,
}

//The current selected button value
pub struct SelectedPausebutton(PauseButtonType);

#[derive(Component)]
pub struct PauseButton(PauseButtonType);

impl Plugin for GamePausePlugin{
    fn build(&self, app: &mut App){
        app
        // When entering the state, spawn everything needed for this screen
        .add_system_set(SystemSet::on_enter(GameLevel::Pause).with_system(setup))
        .add_system_set(SystemSet::on_update(GameState::Game).with_system(pause_pressed))
        // Run those systems on update for each frame
        .add_system_set(SystemSet::on_update(GameLevel::Pause)
            .with_system(mark_selected)
            .with_system(next_previous_button)
            .with_system(select_button)
            .with_system(mouse_on_button)
            )
        // When exiting the state, despawn everything that was spawned for this screen
        .add_system_set(SystemSet::on_exit(GameLevel::Pause).with_system(show_reset_button))
        .add_system_set(SystemSet::on_exit(GameLevel::Pause)
        .with_system(despawn_screen::<OnPauseScreen>),);
    }
}

// spawn everything needed for this screen
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut button_query: Query<Entity, With<ResetButton>>){
    
    let entity = button_query.single_mut();

    commands.entity(entity).despawn_recursive();

    //A semitransparent black container for the menu that covers the game sprites
    let shadow = commands
    .spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(1920.0), Val::Px(1080.0)),
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            ..default()
        },
        color: Color::rgba(0.,0.,0.,0.95).into(),
        transform: Transform::from_translation(Vec3::new(0.0,0.0, 100.0)),
        ..default()
    }).insert(OnPauseScreen).id();
    //All the button names
    let button_texts = vec!["RESUME", "QUIT", ];
    
    //A container entity containing all the buttons
    let node = spawn_menu_container(&mut commands);
    commands.entity(node).insert(OnPauseScreen);

    //Spawn a new button for each string in vector
    for text in button_texts{
        
        //Helper function gets the enum value from text
        let menu_type = menu_item_from_text(text);
        let button = spawn_button(&mut commands, &asset_server, text, menu_type,);
        commands.entity(node).add_child(button);
    }
    commands.insert_resource(SelectedPausebutton(PauseButtonType::ResumeGame));
    commands.entity(shadow).add_child(node);
}

fn pause_pressed(mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameLevel>>,
    mut player_spawned: ResMut<State<PlayerSpawned>>
){
    if keyboard_input.just_pressed(KeyCode::Escape){
        game_state.push(GameLevel::Pause).unwrap();
        player_spawned.push(PlayerSpawned::Paused).unwrap();
        keyboard_input.clear();
    }
}

//Helper function gets the enum value from text
fn menu_item_from_text(text: &str) -> PauseButton {
    match text {
        "RESUME" => {
            PauseButton(PauseButtonType::ResumeGame)
        },
        "QUIT" => {
            PauseButton(PauseButtonType::Quit)
        },
        _ => {
            PauseButton(PauseButtonType::Quit)
        }
    }
}

//Change the selected button scale so you know with is it
fn mark_selected(selected: Res<SelectedPausebutton>, mut query: Query<(&mut Transform, &PauseButton)>){
    for (mut transfom, menu_button) in query.iter_mut(){
        if selected.0 == menu_button.0{
            transfom.scale = Vec3::new(1.2, 1.2, 1.);
        }
        else{
            transfom.scale = Vec3::new(1., 1., 1.);
        }
    }
}

//Move between button values with the arrow keys (or what ever is the up/down keys) 
fn next_previous_button(mut selected: ResMut<SelectedPausebutton>,keyboard_input: Res<Input<KeyCode>>){
    let mut int_selection= selected.0 as u8;

    if keyboard_input.just_pressed(KeyCode::Down) {
        //int_selection = (int_selection + 1) % 5;
        int_selection = if int_selection < 1 { int_selection + 1 }else{ 0 };
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        //int_selection = (int_selection - 1) % 5;
        int_selection = if int_selection > 0 { int_selection - 1 }else{ 1 };
    }
    selected.0 = PauseButtonType::from_u8(int_selection).unwrap();
}

fn select_button(selected: ResMut<SelectedPausebutton>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut level_state: ResMut<State<GameLevel>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut player_spawn_state: ResMut<State<PlayerSpawned>>,

){
    if keyboard_input.just_pressed(KeyCode::Return){
        accept_selection(&selected, &mut level_state, &mut player_spawn_state, &mut app_exit_events);

    }
    else if keyboard_input.just_pressed(KeyCode::Escape)
    {
        level_state.pop().unwrap();
        player_spawn_state.pop().unwrap();
        keyboard_input.clear();
    }
}

fn mouse_on_button(
    mut interaction_query: Query<
        (&Interaction,&PauseButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected: ResMut<SelectedPausebutton>,
    mut level_state: ResMut<State<GameLevel>>,
    mut player_spawn_state: ResMut<State<PlayerSpawned>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, menu_button)  in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                accept_selection(&selected, &mut level_state, &mut player_spawn_state, &mut app_exit_events);
            },
            Interaction::Hovered => {
                selected.0 = menu_button.0;
            },
            Interaction::None => {},
        }
    }
}

fn accept_selection(
    selected: &ResMut<SelectedPausebutton>,
    level_state: &mut ResMut<State<GameLevel>>,
    player_spawn_state: &mut ResMut<State<PlayerSpawned>>,
    app_exit_events: &mut EventWriter<AppExit>,
){
    match selected.0 {
        PauseButtonType::ResumeGame =>{
            level_state.pop().unwrap();
            player_spawn_state.pop().unwrap();
        },
        PauseButtonType::Quit => app_exit_events.send(AppExit),
    }
}

fn show_reset_button(mut button_query: Query<&mut Visibility, With<ResetButton>>){
    let mut visibility = button_query.single_mut();

    visibility.is_visible = true;

}


