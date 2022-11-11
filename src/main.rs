use bevy::{
    core::Zeroable,
    prelude::*,
    render::camera::{self, RenderTarget},
    window::close_on_esc,
};
use bevy_mod_picking::*;
use bevy_mod_raycast::*;
use bevy_text_mesh::prelude::*;

// Exemple to I18n import
use rust_i18n::t;

rust_i18n::i18n!("locales");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
        .add_plugin(TextMeshPlugin)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Bevy Game".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .init_resource::<CameraData>()
        .init_resource::<PlayerData>()
        .add_event::<InteractionStateEvent>()
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(setup)
        .add_startup_system(setup_ui)
        .add_system_set(
            SystemSet::new()
                .with_system(move_camera)
                .with_system(change_camera_data)
                .with_system(set_character_target)
                .with_system(update_ore_ui)
                .with_system(debug_inputs),
        )
        .add_system(move_character_towards_target)
        .add_system(set_interaction_text)
        .add_system(close_on_esc)
        .add_system_to_stage(CoreStage::PostUpdate, select_character_picking_event)
        .run();
}

#[derive(Default)]
struct CameraData {
    speed: f32,
}

#[derive(Default)]
struct PlayerData {
    ore: u32,
    max_ore: u32,
}

#[derive(Component)]
struct MainCamera;

struct MyRaycastSet;

#[derive(Component)]
struct Selected;

#[derive(Component)]
struct Movable {
    speed: f32,
    target: Option<Vec3>,
}

#[derive(Component)]
struct CanInteract {
    state: InteractState,
}

enum InteractState {
    StandBy,
    GoingToInteract,
    Interacting,
}

pub enum CollectibleResourceType {
    Ore,
    None,
}

#[derive(Component)]
struct CollectibleResourceVein {
    resource_type: CollectibleResourceType,
    collect_point: Vec3,
    amount: u32,
}

struct InteractionStateEvent(Entity, InteractState);

fn setup(
    mut commands: Commands,
    mut player_data: ResMut<PlayerData>,
    mut camera_data: ResMut<CameraData>,
    mut window: ResMut<Windows>,
) {
    camera_data.speed = 20.0;
    player_data.ore = 15;
    player_data.max_ore = 25;

    window.primary_mut().set_cursor_lock_mode(true);
    commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera)
        .insert_bundle(PickingCameraBundle::default())
        .insert(RayCastSource::<MyRaycastSet>::new())
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
        .insert(RayCastMesh::<MyRaycastSet>::default()); // Make this mesh ray cast-able;

    // The "workers" cubes
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(Movable {
            speed: 5.0,
            target: None,
        })
        .insert(CanInteract {
            state: InteractState::StandBy,
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
        .insert(Movable {
            speed: 5.0,
            target: None,
        })
        .insert(CanInteract {
            state: InteractState::StandBy,
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
        .insert(Movable {
            speed: 5.0,
            target: None,
        })
        .insert(CanInteract {
            state: InteractState::StandBy,
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
        .insert(CollectibleResourceVein {
            resource_type: CollectibleResourceType::Ore,
            amount: 50,
            collect_point: Vec3 {
                x: -4.0,
                y: 0.0,
                z: 5.0,
            },
        })
        .insert(RayCastMesh::<MyRaycastSet>::default());
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut transforms: Query<&mut Transform, With<Camera3d>>,
    camera_data: Res<CameraData>,
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

fn change_camera_data(keyboard_input: Res<Input<KeyCode>>, mut camera_data: ResMut<CameraData>) {
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
                        commands.entity(*s).remove::<Selected>();
                    }
                    SelectionEvent::JustSelected(s) => {
                        commands.entity(*s).insert(Selected);
                    }
                }
            }
            PickingEvent::Hover(_e) => { /*info!("Egads! A hover event!? {:?}", e)*/ }
            PickingEvent::Clicked(_e) => { /*info!("Gee Willikers, it's a click! {:?}", e)*/ }
        }
    }
}

fn move_character_towards_target(
    mut selectables: Query<(
        Entity,
        &mut Transform,
        &mut Movable,
        Option<&mut CanInteract>,
    )>,
    mut ev_interaction_text: EventWriter<InteractionStateEvent>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut movable, can_interact) in selectables.iter_mut() {
        match movable.target {
            Some(t) => match can_interact {
                Some(mut i)
                    if matches!(i.state, InteractState::GoingToInteract)
                        && (transform.translation - t).length() < 0.2 =>
                {
                    movable.target = None;
                    i.state = InteractState::Interacting;
                    ev_interaction_text
                        .send(InteractionStateEvent(entity, InteractState::Interacting));
                }
                _ => {
                    if (transform.translation - t).length() < 0.2 {
                        movable.target = None;
                    } else {

                        let translate = (t - transform.translation).normalize()
                            * movable.speed
                            * time.delta_seconds();
                        transform.translation += translate;
                    }
                }
            },
            None => continue,
        }
    }
}

// Update our `RayCastSource` with the current cursor position every frame.
fn set_character_target(
    mut query: Query<&mut RayCastSource<MyRaycastSet>>,
    collectibles: Query<&CollectibleResourceVein>,
    mut selecteds: Query<(Entity, &mut Movable, Option<&mut CanInteract>), With<Selected>>,
    mut ev_interaction_text: EventWriter<InteractionStateEvent>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    // Get the main window
    let window = windows.get_primary().unwrap();

    // Check if the cursor is in the main window
    if let Some(cursor_position) = window.cursor_position() {
        for mut pick_source in &mut query {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);

            if buttons.just_pressed(MouseButton::Right) {
                if let Some((entity, intersection)) = pick_source.intersect_top() {
                    let (new_target, interact) = if let Ok(vein) = collectibles.get(entity) {
                        (vein.collect_point, true)
                    } else {
                        (intersection.position(), false)
                    };

                    for (entity, mut movable, can_interact) in selecteds.iter_mut() {
                        movable.target = Some(new_target);

                        match can_interact {
                            Some(mut i) if interact => {
                                i.state = InteractState::GoingToInteract;
                                ev_interaction_text.send(InteractionStateEvent(
                                    entity,
                                    InteractState::GoingToInteract,
                                ));
                            }
                            Some(mut i) => {
                                i.state = InteractState::StandBy;
                                ev_interaction_text
                                    .send(InteractionStateEvent(entity, InteractState::StandBy));
                            }
                            _ => continue,
                        }
                    }
                }
            }
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, player_data: Res<PlayerData>) {
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

fn update_ore_ui(player_data: Res<PlayerData>, mut texts: Query<&mut Text>) {
    for mut text in texts.iter_mut() {
        text.sections[0].value = format!("Ore: {}/{}", player_data.ore, player_data.max_ore);
    }
}

fn debug_inputs(keyboard_input: Res<Input<KeyCode>>, mut player_data: ResMut<PlayerData>) {
    if keyboard_input.just_pressed(KeyCode::P) {
        player_data.ore += 1;
    }
}

fn set_interaction_text(
    mut ev_interaction_text: EventReader<InteractionStateEvent>,
    q_parent: Query<&Children>,
    mut q_child: Query<&mut TextMesh>,
) {
    for ev in ev_interaction_text.iter() {
        if let Ok(component) = q_parent.get(ev.0) {
            for &child in component {
                if let Ok(mut text) = q_child.get_mut(child) {
                    match ev.1 {
                        InteractState::StandBy => text.text = String::from("Not interacting"),
                        InteractState::GoingToInteract => {
                            text.text = String::from("Going to interact")
                        }
                        InteractState::Interacting => text.text = String::from("Interacting"),
                    }
                }
            }
        }
    }
}
