use std::time::Duration;

use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_mod_raycast::RayCastMesh;
use bevy_text_mesh::{TextMeshBundle, TextMesh};

use crate::{components::{self, resource_vein::CollectibleResourceUI, dialog::{dialog_ui::DialogBox, dialog_choice_button::DialogChoiceButton}}, resources};

use super::SceneState;

pub struct STRScenePlugin;

impl Plugin for STRScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::RTSScene).with_system(spawn_basic_scene));
        app.add_system_set(SystemSet::on_enter(SceneState::RTSScene).with_system(setup_ui));
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // The terrain plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(RayCastMesh::<components::selectable::Selectable>::default()); // Make this mesh ray cast-able;

    // The "workers" cubes
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(components::movable::Movable {
            speed: 5.0,
            target: None,
        })
        .insert(components::can_interact::CanInteract {
            state: resources::interact_state::InteractState::StandBy,
        })
        .with_children(|parent| {
          // Test 3D text
          let font = asset_server.load("fonts/FiraMono-Medium.ttf#mesh");

          parent.spawn_bundle(TextMeshBundle {
              text_mesh: TextMesh::new_with_color("Hello Bevy", font, Color::rgb(0., 0., 1.)),
              transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -90., 0., 0.))
                  .with_translation(Vec3::new(-1., 1., 0.)),
              ..Default::default()
          });
      });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(2.0, 0.5, 2.0),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(components::movable::Movable {
            speed: 5.0,
            target: None,
        })
        .insert(components::can_interact::CanInteract {
            state: resources::interact_state::InteractState::StandBy,
        })
        .with_children(|parent| {
          // Test 3D text
          let font = asset_server.load("fonts/FiraMono-Medium.ttf#mesh");

          parent.spawn_bundle(TextMeshBundle {
              text_mesh: TextMesh::new_with_color("Hello Bevy", font, Color::rgb(0., 0., 1.)),
              transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -90., 0., 0.))
                  .with_translation(Vec3::new(-1., 1., 0.)),
              ..Default::default()
          });
      });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-1.0, 0.5, 1.0),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(components::movable::Movable {
            speed: 5.0,
            target: None,
        })
        .insert(components::can_interact::CanInteract {
            state: resources::interact_state::InteractState::StandBy,
        })
        .with_children(|parent| {
            // Test 3D text
            let font = asset_server.load("fonts/FiraMono-Medium.ttf#mesh");

            parent.spawn_bundle(TextMeshBundle {
                text_mesh: TextMesh::new_with_color("Hello Bevy", font, Color::rgb(0., 0., 1.)),
                transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -90., 0., 0.))
                    .with_translation(Vec3::new(-1., 1., 0.)),
                ..Default::default()
            });
        });

    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 20.0, 20.0),
            ..default()
        }
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // The resource vein
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                ..default()
            })),
            material: materials.add(Color::rgb(0.97, 0.24, 0.22).into()),
            transform: Transform::from_xyz(-5.0, 0., 5.0),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(components::resource_vein::ResourceVeinComponent {
            resource_type: resources::resources_enums::InGameResourceType::Ore,
            amount: 50,
            workers: Vec::new(),
            timer: Timer::new(Duration::from_millis(500), true),
        })
        .insert(RayCastMesh::<components::selectable::Selectable>::default())
        .insert(components::interactible::Interactible { interaction_point: Vec3 {x: -4., y: 0., z: 5.}})
        .with_children(|parent| {
            // Test 3D text
            let font = asset_server.load("fonts/FiraMono-Medium.ttf#mesh");
  
            parent.spawn_bundle(TextMeshBundle {
                text_mesh: TextMesh::new_with_color("Remaining ore : 50", font, Color::rgb(0., 0., 1.)),
                transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -90., 0., 0.))
                    .with_translation(Vec3::new(-1., 1., 0.)),
                ..Default::default()
            });
        });
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, player_data: Res<resources::player::data::PlayerData>) {
    commands.spawn_bundle(
        TextBundle::from_sections([TextSection::new(
            format!("Ore: {}/{}", player_data.ore(), player_data.max_ore),
            TextStyle {
                font: asset_server.load("fonts/Akira Expanded Demo.otf"),
                font_size: 20.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
    )
    .insert(CollectibleResourceUI);


    // Dialog box (borders)
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(1280.0), Val::Px(300.0)),
            position_type: PositionType::Absolute,
            border: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        color: Color::rgb(0.4, 0.4, 1.0).into(),
        ..default()
    })
    .insert(DialogBox)
    .insert(Visibility {
        is_visible: false
    })

    // Inner dialog box
    .with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: Color::rgb(0.8, 0.8, 1.0).into(),
            ..default()
        })
        .with_children(|parent| {

            // Text
            parent.spawn_bundle(
                TextBundle::from_sections([TextSection::new(
                    format!(""),
                    TextStyle {
                        font: asset_server.load("fonts/Akira Expanded Demo.otf"),
                        font_size: 20.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                )])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(20.),
                        left: Val::Px(20.),
                        ..default()
                    },
                    align_content: AlignContent::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                }),
            )
            .insert(components::dialog::dialog_ui::DialogText);

            // Next buttons
            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: UiRect {
                        bottom: Val::Px(5.),
                        top: Val::Auto,
                        right: Val::Auto,
                        left: Val::Auto,
                    },
                    // horizontally center child text
                    justify_content: JustifyContent::FlexEnd,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: bevy::prelude::UiColor(Color::rgb(0.15, 0.15, 0.85)),
                ..default()
            })
            .with_children(|parent| {
                // Button text "Next"
                parent.spawn_bundle(
                    TextBundle::from_sections([TextSection::new(
                        format!("Next"),
                        TextStyle {
                            font: asset_server.load("fonts/Akira Expanded Demo.otf"),
                            font_size: 20.0,
                            color: Color::rgb(0.5, 0.5, 1.0),
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
            .insert(DialogChoiceButton {id:0, enabled: true});

            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: UiRect {
                        bottom: Val::Px(5.),
                        top: Val::Auto,
                        right: Val::Auto,
                        left: Val::Auto,
                    },
                    // horizontally center child text
                    justify_content: JustifyContent::FlexEnd,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: bevy::prelude::UiColor(Color::rgb(0.15, 0.15, 0.85)),
                ..default()
            })
            .with_children(|parent| {
                // Button text "Next"
                parent.spawn_bundle(
                    TextBundle::from_sections([TextSection::new(
                        format!("Next"),
                        TextStyle {
                            font: asset_server.load("fonts/Akira Expanded Demo.otf"),
                            font_size: 20.0,
                            color: Color::rgb(0.5, 0.5, 1.0),
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
            .insert(DialogChoiceButton {id:1, enabled: true});

            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: UiRect {
                        bottom: Val::Px(5.),
                        top: Val::Auto,
                        right: Val::Auto,
                        left: Val::Auto,
                    },
                    // horizontally center child text
                    justify_content: JustifyContent::FlexEnd,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: bevy::prelude::UiColor(Color::rgb(0.15, 0.15, 0.85)),
                ..default()
            })
            .with_children(|parent| {
                // Button text "Next"
                parent.spawn_bundle(
                    TextBundle::from_sections([TextSection::new(
                        format!("Next"),
                        TextStyle {
                            font: asset_server.load("fonts/Akira Expanded Demo.otf"),
                            font_size: 20.0,
                            color: Color::rgb(0.5, 0.5, 1.0),
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
            .insert(DialogChoiceButton {id:2, enabled: true});
        });
    });
}