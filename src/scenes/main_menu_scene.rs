use std::slice::Iter;

use bevy::{prelude::*, sprite::Rect, app::AppExit};

use crate::materials::menu_box::MenuBoxMaterials;

use super::SceneState;

const MAIN_MENU_BOX_ARRAY: [[i8; 5]; 8] = [
    [0, 1, 1, 1, 2],
    [3, 4, 4, 4, 5],
    [3, 4, 4, 4, 5],
    [3, 4, 4, 4, 5],
    [3, 4, 4, 4, 5],
    [3, 4, 4, 4, 5],
    [3, 4, 4, 4, 5],
    [6, 7, 7, 7, 8],
];
const FONT_SIZE: f32 = 36.0;
const MAIN_MENU_BOX_TILE_SIZE: f32 = 50.0;

#[derive(Component, Copy, Clone)]
enum ButtonComponent {
    Play,
    Highscore,
    Options,
    Help,
    Credits,
    Quit,
}

impl ButtonComponent {
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::Play,
            ButtonComponent::Highscore,
            ButtonComponent::Options,
            ButtonComponent::Help,
            ButtonComponent::Credits,
            ButtonComponent::Quit,
        ]
        .iter()
    }
}

struct MainMenuSceneData {
    user_interface_root: Entity,
}

pub struct MainMenuScenePlugin;

impl Plugin for MainMenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::MainMenuScene).with_system(setup));
        app.add_system_set(SystemSet::on_exit(SceneState::MainMenuScene).with_system(cleanup));
        app.add_system_set(
            SystemSet::on_update(SceneState::MainMenuScene).with_system(button_handle_system),
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let menu_box_materials: MenuBoxMaterials = MenuBoxMaterials{
        top_right: asset_server.load("scenes/gui/menu_box/top_right.png"),
        top_center: asset_server.load("scenes/gui/menu_box/top_center.png"),
        top_left: asset_server.load("scenes/gui/menu_box/top_left.png"),
        mid_right: asset_server.load("scenes/gui/menu_box/mid_right.png"),
        mid_center: asset_server.load("scenes/gui/menu_box/mid_center.png"),
        mid_left: asset_server.load("scenes/gui/menu_box/mid_left.png"),
        bottom_right: asset_server.load("scenes/gui/menu_box/bottom_right.png"),
        bottom_center: asset_server.load("scenes/gui/menu_box/bottom_center.png"),
        bottom_left: asset_server.load("scenes/gui/menu_box/bottom_left.png"),
    };
    
    let user_interface_root = commands
    .spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        image: UiImage(asset_server.load("images/main_menu_background.png")),
        ..Default::default()
    })
    .with_children(|parent| {
        main_menu_box(parent, &menu_box_materials);
        buttons(parent, asset_server);
    })
    .id();
    
    commands.insert_resource(menu_box_materials);
    commands.insert_resource(MainMenuSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, main_menu_scene_data: Res<MainMenuSceneData>) {
    commands
        .entity(main_menu_scene_data.user_interface_root)
        .despawn_recursive();
}

fn main_menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let size: Size<Val> = Size {
        width: Val::Px(MAIN_MENU_BOX_TILE_SIZE),
        height: Val::Px(MAIN_MENU_BOX_TILE_SIZE),
    };

    for (row_index, row) in MAIN_MENU_BOX_ARRAY.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let position: UiRect<Val> = UiRect {
                left: Val::Px(10.0 + MAIN_MENU_BOX_TILE_SIZE * column_index as f32),
                top: Val::Px(150.0 + MAIN_MENU_BOX_TILE_SIZE * row_index as f32),
                bottom: Val::Auto,
                right: Val::Auto,
            };

            let image: Handle<Image> = match value {
                0 => menu_box_materials.top_right.clone(),
                1 => menu_box_materials.top_center.clone(),
                2 => menu_box_materials.top_left.clone(),
                3 => menu_box_materials.mid_right.clone(),
                4 => menu_box_materials.mid_center.clone(),
                5 => menu_box_materials.mid_left.clone(),
                6 => menu_box_materials.bottom_right.clone(),
                7 => menu_box_materials.bottom_center.clone(),
                8 => menu_box_materials.bottom_left.clone(),
                _ => panic!("Unknown resources"),
            };

            root.spawn_bundle(NodeBundle {
                image: UiImage(image),
                style: Style {
                    position_type: PositionType::Absolute,
                    position,
                    size,
                    ..Default::default()
                },

                ..Default::default()
            });
        }
    }
}

fn buttons(
    root: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
) {
    for (index, button) in ButtonComponent::iterator().enumerate() {
        let position: UiRect<Val> = UiRect {
            left: Val::Px(10.0 + MAIN_MENU_BOX_TILE_SIZE * (3.0 - 1.0) / 2.0),
            right: Val::Auto,
            top: Val::Px(150.0 + MAIN_MENU_BOX_TILE_SIZE * (index as f32 + 1.0)),
            bottom: Val::Auto,
        };

        let size = Size {
            width: Val::Px(MAIN_MENU_BOX_TILE_SIZE * 3.0),
            height: Val::Px(MAIN_MENU_BOX_TILE_SIZE),
        };

        root.spawn_bundle(ButtonBundle {
            style: Style {
                size,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexEnd,
                position,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            let text: &str = match button {
                ButtonComponent::Play => "Play",
                ButtonComponent::Highscore => "Highscore",
                ButtonComponent::Options => "Options",
                ButtonComponent::Help => "Help",
                ButtonComponent::Credits => "Credits",
                ButtonComponent::Quit => "Quit",
            };

            parent.spawn_bundle(
                TextBundle::from_sections([TextSection::new(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::BLACK,
                    },

                )])
                .with_style(Style {
                    margin: UiRect::all(Val::Auto),
                    align_content: AlignContent::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                }),
            );
        })
        .insert(button.clone());
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &ButtonComponent, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<SceneState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button, children) in button_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => text.sections[0].style.color = Color::GRAY,
            Interaction::Hovered => text.sections[0].style.color = Color::BLACK,
            Interaction::Clicked => {
                text.sections[0].style.color = Color::RED;
                match button {
                    ButtonComponent::Play => state
                        .set(SceneState::RTSScene)
                        .expect("Couldn't switch state to RTSScene"),
                        ButtonComponent::Highscore => println!("Not implemented yet"),
                        ButtonComponent::Options => println!("Not implemented yet"),
                        ButtonComponent::Help => println!("Not implemented yet"),
                        ButtonComponent::Credits => println!("Not implemented yet"),
                        ButtonComponent::Quit => exit.send(AppExit),
                }
            }
        }
    }
}
