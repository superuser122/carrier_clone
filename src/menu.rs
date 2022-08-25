use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use bevy::{prelude::*, app::AppExit,};
use super::{despawn_screen, GameState, GameLevel, ui::* };


pub struct MenuPlugin;
//Enum values representing all the menu buttons
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, FromPrimitive)]
pub enum MenuButtonType{
    Start,
    Quit,
}

//The current selected button value
pub struct SelectedButton(MenuButtonType);

//A componet holdin the value for each button 
#[derive(Component)]
pub struct MenuButton(MenuButtonType);

//Helps to despawn entities
#[derive(Component)]
struct OnMenuScreen;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App){
        app
         // When entering the state, spawn everything needed for this screen
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup))
        
        // Run those systems on update for each frame
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(mark_selected))
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(next_previous_button))
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(select_button))
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(mouse_on_button))

        // When exiting the state, despawn everything that was spawned for this screen
        .add_system_set(SystemSet::on_exit(GameState::Menu)
            .with_system(despawn_screen::<OnMenuScreen>),);
    }
}

// spawn everything needed for this screen
fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    //All the button names
    let button_texts = vec!["START", "QUIT"];

    //A container entity containing all the buttons
    let node = spawn_menu_container(&mut commands);
    commands.entity(node).insert(OnMenuScreen);
   
    //Spawn a new button for each string in vector
    for text in button_texts{
        
        //Helper function gets the enum value from text
        let menu_type = menu_item_from_text(text);
        let button = spawn_button(&mut commands, &asset_server, text, menu_type);
        commands.entity(node).add_child(button);
    }
    //Insert the resource with default value
    commands.insert_resource(SelectedButton(MenuButtonType::Start));
}

//Helper function gets the enum value from text
fn menu_item_from_text(text: &str) -> MenuButton {
    match text {
        "START" => {
            MenuButton(MenuButtonType::Start)
        },
        "QUIT" => {
            MenuButton(MenuButtonType::Quit)
        },

        _ => {
            MenuButton(MenuButtonType::Quit)
        }
    }
}

//Change the selected button scale so you know with is it
fn mark_selected(selected: Res<SelectedButton>, mut query: Query<(&mut Transform, &MenuButton)>){
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
fn next_previous_button(mut selected: ResMut<SelectedButton>,keyboard_input: Res<Input<KeyCode>>){
    
    let mut int_selection= selected.0 as u8;

    if keyboard_input.just_pressed(KeyCode::Down) {
        //int_selection = (int_selection + 1) % 5;
        int_selection = if int_selection < 1 { int_selection + 1 }else{ 0 };
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        //int_selection = (int_selection - 1) % 5;
        int_selection = if int_selection > 0 { int_selection - 1 }else{ 1 };
    }
    selected.0 = MenuButtonType::from_u8(int_selection).unwrap();
}

fn select_button(selected: ResMut<SelectedButton>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
){
    if keyboard_input.just_pressed(KeyCode::Return){
        accept_selection(&selected, &mut game_state,  &mut app_exit_events);

    }
}

fn mouse_on_button(
    mut interaction_query: Query<
        (&Interaction,&MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected: ResMut<SelectedButton>,
    mut game_state: ResMut<State<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, menu_button)  in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                accept_selection(&selected, &mut game_state, &mut app_exit_events);
            },
            Interaction::Hovered => {
                selected.0 = menu_button.0;
            },
            Interaction::None => {},
        }
    }
}

//Called whene a button was accepted either by mouse click or by pressing enter
fn accept_selection(
    selected: &ResMut<SelectedButton>,
    game_state: &mut ResMut<State<GameState>>,
    app_exit_events: &mut EventWriter<AppExit>,
){
    match selected.0 {
        MenuButtonType::Start => {
            game_state.replace(GameState::Game).unwrap();
        },
        MenuButtonType::Quit => app_exit_events.send(AppExit),
        _ => {}
    }

}