use bevy::prelude::*;
use std::time::Duration;
use crate::types::*;

#[derive(Component)]
struct Planet {
    speed: Speed,
    direction: Directions,
}

struct SpawnTimer(Timer);

fn spawn_planet(
  mut commands: Commands,
  materials: Res<Assets<StandardMaterial>>,
  time: Res<Time>,
) {
  let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
  let mut repeating = Timer::from_seconds(1.0, TimerMode::Repeating);
  timer.tick(Duration::from_secs_f32(1.5));
  repeating.tick(Duration::from_secs_f32(1.5));
  assert_eq!(timer.elapsed_secs(), 1.0);
  assert_eq!(repeating.elapsed_secs(), 0.5);
}

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .add_startup_system(spawn_planet)
      .add_system(spawn_planet);
  }
}