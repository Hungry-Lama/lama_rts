pub mod loading_scene;
pub mod main_menu_scene;
pub mod str_scene;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneState {
    MainMenuScene,
    RTSScene,
}
