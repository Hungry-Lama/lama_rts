use bevy::prelude::*;
use bevy_mod_picking::{PickingCameraBundle, RayCastSource};
use bevy_mod_raycast::DefaultPluginState;

use crate::{resources::{camera::CameraData, self}, components::{camera::MainCamera, self}};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<CameraData>()
        .add_startup_system(setup)
        .add_system(move_camera)
        .add_system(change_camera_data);
    }
}

pub fn setup(
    mut commands: Commands,
) {
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
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut transforms: Query<&mut Transform, With<Camera3d>>,
    camera_data: Res<resources::camera::CameraData>,
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

fn change_camera_data(keyboard_input: Res<Input<KeyCode>>, mut camera_data: ResMut<resources::camera::CameraData>) {
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