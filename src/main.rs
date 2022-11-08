use bevy::{window::close_on_esc, prelude::*, render::camera::{self, RenderTarget}};
use bevy_mod_picking::*;
use bevy_mod_raycast::*;

// Exemple to I18n import
use rust_i18n::t;

rust_i18n::i18n!("locales");


fn main () {
  App::new()
  .add_plugins(DefaultPlugins)
  .add_plugins(DefaultPickingPlugins)
  .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
  .insert_resource(Msaa { samples: 4 })
  .insert_resource(WindowDescriptor {
    title: "Bevy Game".to_string(),
    width: 1280.,
    height: 720.,
    ..Default::default()
  })
  .init_resource::<CameraData>()
  .add_startup_system(spawn_basic_scene)
  .add_startup_system(setup)
  .add_system_set(
    SystemSet::new()
    .with_system(move_camera)
    .with_system(change_camera_data)
    .with_system(set_character_target),
  )
  .add_system(move_character_towards_target)
  .add_system(close_on_esc)
  .add_system_to_stage(CoreStage::PostUpdate, select_character_picking_event)
  .run();
}

#[derive(Default)]
struct CameraData {
    speed: f32,
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

fn setup(
    mut commands: Commands,
    mut camera: ResMut<CameraData>,
    mut window: ResMut<Windows>
) {
  camera.speed = 20.0;
  window.primary_mut().set_cursor_lock_mode(true);
  commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());

  commands
    .spawn_bundle(Camera3dBundle{
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
) {
  commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Plane { size: 25.0 })),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      ..default()
  }).insert(RayCastMesh::<MyRaycastSet>::default()); // Make this mesh ray cast-able;

  commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
  })
  .insert_bundle(PickableBundle::default())
  .insert(Movable {
    speed: 5.0,
    target: None,
  });

  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
    transform: Transform::from_xyz(2.0, 0.5, 2.0),
    ..default()
  })
  .insert_bundle(PickableBundle::default())
  .insert(Movable {
    speed: 5.0,
    target: None,
  });

  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
    transform: Transform::from_xyz(-1.0, 0.5, 1.0),
    ..default()
  })
  .insert_bundle(PickableBundle::default())
  .insert(Movable {
    speed: 5.0,
    target: None,
  });


  commands.spawn_bundle(PointLightBundle {
    transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0)),
    ..Default::default()
});
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
  let screen_pos = window.cursor_position().unwrap();
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

fn change_camera_data(
  keyboard_input: Res<Input<KeyCode>>,
  mut camera_data: ResMut<CameraData>,
) {
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
  mut events: EventReader<PickingEvent>
) {
  for event in events.iter() {
      match event {
          PickingEvent::Selection(e) => {
            //info!("A selection event happened: {:?}", e);

            match e {
              SelectionEvent::JustDeselected(s) => {
                commands.entity(*s).remove::<Selected>();
              },
              SelectionEvent::JustSelected(s) => {
                commands.entity(*s).insert(Selected);
              }
            }
          },
          PickingEvent::Hover(e) => {/*info!("Egads! A hover event!? {:?}", e)*/},
          PickingEvent::Clicked(e) => {/*info!("Gee Willikers, it's a click! {:?}", e)*/},
      }
  }
}

fn move_character_towards_target(
  mut selectables: Query<(&mut Transform, &mut Movable)>,
  time: Res<Time>,
) {
  for (mut transform, mut movable) in selectables.iter_mut() {
    match movable.target {
      Some(t) => {

        if (transform.translation - t).length() < 0.2 {
          movable.target = None;
        } else {
          let translate = (t - transform.translation).normalize() * movable.speed * time.delta_seconds();
          transform.translation += translate;
        }
      },
      None => continue,
    }
  }
}

// Update our `RayCastSource` with the current cursor position every frame.
fn set_character_target(
  mut query: Query<&mut RayCastSource<MyRaycastSet>>,
  mut selectables: Query<&mut Movable, With<Selected>>,
  buttons: Res<Input<MouseButton>>,
  windows: Res<Windows>,
) {
  let window = windows.get_primary().unwrap();

  if let Some(cursor_position) = window.cursor_position() {
    for mut pick_source in &mut query {
      pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);

      if buttons.just_pressed(MouseButton::Right) {
        if let Some((_, intersection)) = pick_source.intersect_top() {
          for mut movable in selectables.iter_mut() {
            let new_target = intersection.position();
            movable.target = Some(new_target);
          }
        }
      }
    }
  }
}