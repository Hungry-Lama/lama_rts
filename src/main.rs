use bevy::prelude::*;
use bevy_mod_picking::*;
use bevy_mod_raycast::*;
use bevy_text_mesh::prelude::*;

mod components;
mod materials;
mod resources;
mod plugins;
mod scenes;

rust_i18n::i18n!("locales");

fn main() {
    App::new()
        .add_state(scenes::SceneState::MainMenuScene)
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DefaultRaycastingPlugin::<components::selectable::Selectable>::default())
        .add_plugin(TextMeshPlugin)
        .add_plugin(plugins::character::CharacterPlugin)
        .add_plugin(plugins::dialog::DialogPlugin)
        .add_plugin(plugins::player::PlayerPlugin)
        .add_plugin(plugins::camera::CameraPlugin)
        .add_plugin(plugins::resource_vein::ResourceVeinPlugin)
        .add_plugin(plugins::selection::SelectionPlugin)
        .add_plugin(plugins::window::WindowPlugin)
        .add_plugin(scenes::str_scene::STRScenePlugin)
        .add_plugin(scenes::main_menu_scene::MainMenuScenePlugin)

        .add_system(debug_inputs)
        .run();
}

fn debug_inputs(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_data: ResMut<resources::player::data::PlayerData>,
    mut current_dialog: ResMut<resources::dialog::CurrentDialog>,
    datas: ResMut<resources::dialog::DialogDatas>
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        player_data.add_ore(1);
    }
    if keyboard_input.just_pressed(KeyCode::D) {
        current_dialog.dialog = datas.dialogs.get(&0).cloned();
    }
    if keyboard_input.just_pressed(KeyCode::F) {
        plugins::dialog::utility::create_json();
    }
}