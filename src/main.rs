use std::time::Duration;

use bevy::{
    prelude::*,
    window::close_on_esc,
};
use bevy_mod_picking::*;
use bevy_mod_raycast::*;
use bevy_text_mesh::prelude::*;

mod components;
mod resources;
mod plugins;

rust_i18n::i18n!("locales");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DefaultRaycastingPlugin::<components::selectable::Selectable>::default())
        .add_plugin(TextMeshPlugin)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Bevy Game".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .init_resource::<components::datas::CameraData>()
        .init_resource::<components::datas::PlayerData>()
        .add_event::<InteractionStateEvent>()
        .add_event::<InteractionStartsEvent>()
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(setup)
        .add_startup_system(setup_ui)
        .add_system_set(
            SystemSet::new()
                .with_system(move_camera)
                .with_system(change_camera_data)
                .with_system(plugins::character::movement::set_character_target)
                .with_system(update_ore_ui)
                .with_system(debug_inputs),
        )
        .add_system(plugins::character::movement::move_towards_target)
        .add_system(plugins::character::interaction::starts_interaction_event)
        .add_system(plugins::character::interaction::set_interaction)
        .add_system(plugins::character::interaction::set_interaction_text)
        .add_system_to_stage(CoreStage::PostUpdate, plugins::resource_vein::collect_resource)
        .add_system_to_stage(CoreStage::PostUpdate, select_character_picking_event)
        .add_system(close_on_esc)
        .run();
}

#[derive(Component)]
struct MainCamera;

pub struct InteractionStateEvent(Entity, resources::interact_state::InteractState);
pub struct InteractionStartsEvent(Entity, Entity);


fn setup(
    mut commands: Commands,
    mut player_data: ResMut<components::datas::PlayerData>,
    mut camera_data: ResMut<components::datas::CameraData>,
    mut window: ResMut<Windows>,
) {
    camera_data.speed = 20.0;
    player_data.ore = 15;
    player_data.max_ore = 25;

    window.primary_mut().set_cursor_lock_mode(true);
    commands.insert_resource(DefaultPluginState::<components::selectable::Selectable>::default().with_debug_cursor());

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera)
        .insert_bundle(PickingCameraBundle::default())
        .insert(RayCastSource::<components::selectable::Selectable>::new())
        .commands();
    // .spawn_bundle(UiCameraBundle::default());
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
            resource_type: resources::collectible_resource_type::CollectibleResourceType::Ore,
            amount: 50,
            workers: Vec::new(),
            timer: Timer::new(Duration::from_millis(500), true),
        })
        .insert(RayCastMesh::<components::selectable::Selectable>::default())
        .insert(components::interactible::Interactible { interaction_point: Vec3 {x: -4., y: 0., z: 5.}});
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut transforms: Query<&mut Transform, With<Camera3d>>,
    camera_data: Res<components::datas::CameraData>,
    time: Res<Time>,
) {
    let mut translation = Vec3::default();

    let window = windows.primary();
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        if keyboard_input.pressed(KeyCode::Up) || ndc.y > 0.95 {
            translation.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) || ndc.y < -0.95 {
            translation.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) || ndc.x > 0.95 {
            translation.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Left) || ndc.x < -0.95 {
            translation.x -= 1.0;
        }

        translation *= time.delta_seconds() * camera_data.speed;

        for mut transform in transforms.iter_mut() {
            transform.translation += translation;
        }
    }
}

fn change_camera_data(keyboard_input: Res<Input<KeyCode>>, mut camera_data: ResMut<components::datas::CameraData>) {
    if keyboard_input.just_pressed(KeyCode::NumpadAdd) {
        camera_data.speed += 5.0;
    }
    if keyboard_input.just_pressed(KeyCode::NumpadSubtract) {
        camera_data.speed -= 5.0;
    }

    if camera_data.speed > 40.0 {
        camera_data.speed = 40.0;
    } else if camera_data.speed < 5.0 {
        camera_data.speed = 5.0;
    }
}

pub fn select_character_picking_event(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => {
                //info!("A selection event happened: {:?}", e);

                match e {
                    SelectionEvent::JustDeselected(s) => {
                        commands.entity(*s).remove::<components::selectable::Selected>();
                    }
                    SelectionEvent::JustSelected(s) => {
                        commands.entity(*s).insert(components::selectable::Selected);
                    }
                }
            }
            PickingEvent::Hover(_e) => { /*info!("Egads! A hover event!? {:?}", e)*/ }
            PickingEvent::Clicked(_e) => { /*info!("Gee Willikers, it's a click! {:?}", e)*/ }
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, player_data: Res<components::datas::PlayerData>) {
    commands.spawn_bundle(
        TextBundle::from_sections([TextSection::new(
            format!("Ore: {}/{}", player_data.ore, player_data.max_ore),
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
    );
}

fn update_ore_ui(player_data: Res<components::datas::PlayerData>, mut texts: Query<&mut Text>) {
    for mut text in texts.iter_mut() {
        text.sections[0].value = format!("Ore: {}/{}", player_data.ore, player_data.max_ore);
    }
}

fn debug_inputs(keyboard_input: Res<Input<KeyCode>>, mut player_data: ResMut<components::datas::PlayerData>) {
    if keyboard_input.just_pressed(KeyCode::P) {
        player_data.ore += 1;
    }
}