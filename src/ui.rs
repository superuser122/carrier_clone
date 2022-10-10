use bevy::prelude::*;

const BUTTON_COLOR: Color = Color::rgb(0.53, 0.38, 0.2);

#[derive(Component)]
pub struct ResetButton;

pub fn spawn_menu_container(commands: &mut Commands) -> Entity{
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect{ top:Val::Px(450.),left:Val::Auto, bottom: Val::Auto, right: Val::Auto},
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor(Color::rgba(0.1, 0.1, 0.1, 0.0)),
            ..default()
        }).id()
}

pub fn spawn_button(commands: &mut Commands, asset_server: &Res<AssetServer>, text: &str, comp: impl Component,) -> Entity{
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(192.0), Val::Px(48.0)),
            // center button
            margin: UiRect::all(Val::Px(12.)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        color: BUTTON_COLOR.into(),
        ..default()
    })
    .insert(comp)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/retganon.ttf"),
                font_size: 30.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));
    }).id()
}

