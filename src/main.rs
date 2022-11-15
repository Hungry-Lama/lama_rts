use bevy::{window::close_on_esc, prelude::*};

mod types;

mod sound;
use sound::start_background_music;

mod planet;
use planet::PlanetPlugin;

fn main () {
    App::new()
      .add_plugins(DefaultPlugins.set(WindowPlugin {
          window: WindowDescriptor {
          title: "Bevy Game".to_string(),
          width: 1280.,
          height: 720.,
          ..default()
        },
        ..default()
      }))
      .add_startup_system(start_background_music)
      .add_plugin(PlanetPlugin)
      .add_startup_system(setup)
      .add_system(close_on_esc)
    .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

  // Camera
  commands.spawn(Camera3dBundle{
    transform: Transform::from_xyz(10.0,30.0, 15.0)
        .looking_at(Vec3::new(5.0, 10.0, 0.0), Vec3::Y),
    ..Default::default()
  });

  // Plane
  commands
    .spawn(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Plane { size: 500000.0 })),
      material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
      ..default()
    });

  // Light
  commands
    .spawn(DirectionalLightBundle {
      transform: Transform::from_rotation(Quat::from_euler(
          EulerRot::ZYX,
          0.0,
          1.0,
          -std::f32::consts::FRAC_PI_4,
      )),
      directional_light: DirectionalLight {
          shadows_enabled: true,
          ..default()
      },
      ..default()
    });

    // Cube
    scene_spawner.spawn(asset_server.load("models/spaceships/craft_cargoA.glb#Scene0"));
}