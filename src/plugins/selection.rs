use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, SelectionEvent};

use crate::components;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage(CoreStage::PostUpdate, select_character_picking_event);
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