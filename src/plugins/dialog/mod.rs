use bevy::prelude::{Plugin, App};

use crate::resources::dialog::*;

pub mod dialog;
pub mod utility;

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DialogDatas>()
        .init_resource::<DialogData>()
        .init_resource::<CurrentDialog>()
        .add_event::<dialog::ReadNextDialog>()
        .add_startup_system(utility::load_json)
        .add_system(dialog::goto_dialog)
        .add_system(dialog::display_current_dialog)
        .add_system(dialog::button_choice_dialog);
    }
}