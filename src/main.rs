use bevy::{window::close_on_esc, prelude::*, render::camera};
// Exemple to I18n import
use rust_i18n::t;

rust_i18n::i18n!("locales");


fn main () {
  App::new()
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
    .with_system(change_camera_data),
  )
  .add_system(close_on_esc)
  .add_plugins(DefaultPlugins)
  .run();
}

#[derive(Default)]
struct CameraData {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    mut camera: ResMut<CameraData>
) {
  camera.speed = 2.0;

  commands
    .spawn_bundle(Camera3dBundle{
      transform: Transform::from_xyz(0.0, 15.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
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
  });

  commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
  });
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
    transform: Transform::from_xyz(2.0, 0.5, 2.0),
    ..default()
  });
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
    transform: Transform::from_xyz(-1.0, 0.5, 1.0),
    ..default()
  });
}

fn move_camera(
  keyboard_input: Res<Input<KeyCode>>,
  mut transforms: Query<&mut Transform, With<Camera3d>>,
  camera_data: Res<CameraData>,
  time: Res<Time>,
) {
  let mut translation = Vec3::default();

  if keyboard_input.pressed(KeyCode::Up) {
      translation.z -= 1.0;
  }
  if keyboard_input.pressed(KeyCode::Down) {
      translation.z += 1.0;
  }
  if keyboard_input.pressed(KeyCode::Right) {
      translation.x += 1.0;
  }
  if keyboard_input.pressed(KeyCode::Left) {
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