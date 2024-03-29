
use bevy::prelude::*;

use crate::{resources::{dialog::{DialogChoiceEvent::*, DialogDatas, CurrentDialog, DialogCondition, DialogConsequences}, player::data::PlayerData}, components::dialog::{dialog_ui::DialogBox, dialog_choice_button::DialogChoiceButton}};
use rand::prelude::*;
use crate::components::dialog::dialog_ui::DialogText;


pub struct ReadNextDialog(Option<u32>);

pub fn goto_dialog (
    datas: Res<DialogDatas>,
    mut current: ResMut<CurrentDialog>,
    mut events: EventReader<ReadNextDialog>,
) {
    for ev in events.iter() {
        if let Some(next) = ev.0 {
            current.dialog = datas.dialogs.get(&next).cloned();
        } else {
            current.dialog = None;
        }
    }
}

pub fn display_current_dialog (
    current: Res<CurrentDialog>,
    player_data: Res<PlayerData>,
    mut dialog_box: Query<&mut Visibility, (With<DialogBox>, Without<DialogChoiceButton>)>,
    mut dialog_text: Query<&mut Text, With<DialogText>>,
    mut dialog_buttons: Query<(&mut Visibility, &mut DialogChoiceButton, &mut UiColor, &Children)>,
    mut dialog_button_text: Query<&mut Text, Without<DialogText>>,
) {
    if !current.is_changed() {
        return;
    }
    
    if let Ok(mut visibility) = dialog_box.get_single_mut() {
        if let Ok(mut text) = dialog_text.get_single_mut() {
            if let Some(dialog) = &current.dialog {
                visibility.is_visible = true;
                text.sections[0].value = format!("{}", dialog.text);

                for (mut btn_visibility, mut button, mut color, children) in dialog_buttons.iter_mut() {
                    if let Some(choice) = dialog.choices.get(button.id) {
                        for &child in children.iter() {
                            if let Ok(mut btn_text) = dialog_button_text.get_mut(child) {
                                btn_text.sections[0].value = choice.text.clone();
                            }
                        }

                        if let Some(conditions) = &choice.conditions {
                            let mut conditions_passed = true;
                            for condition in conditions {
                                match condition {
                                    DialogCondition::ResourceCheck(resource, amount) => {
                                        if player_data.ore() < *amount {
                                            conditions_passed = false;
                                            break;
                                        }
                                    },
                                    DialogCondition::SwitchCheck(switch, state) => todo!(),
                                }
                            };

                            button.enabled = conditions_passed;
                            if conditions_passed {
                                *color = bevy::prelude::UiColor(Color::rgb(0.15, 0.15, 0.85));
                            } else {
                                *color = bevy::prelude::UiColor(Color::rgb(0.5, 0.5, 0.5));
                            }

                        } else {
                            button.enabled = true;
                        }
                        btn_visibility.is_visible = true;
                    } else {
                        btn_visibility.is_visible = false;
                    }
                }

            } else {
                visibility.is_visible = false;
                text.sections[0].value = format!("");
            }
        }
    }
}

pub fn button_choice_dialog(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &DialogChoiceButton),
        (Changed<Interaction>, With<Button>),
    >,
    current: ResMut<CurrentDialog>,
    mut player_datas: ResMut<PlayerData>,
    mut ev: EventWriter<ReadNextDialog>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        if button.enabled == false {
            continue;
        }
        match *interaction {
            Interaction::Clicked => {
                if let Some(dialog) = &current.dialog {
                    if let Some(choice) = dialog.choices.get(button.id) {
                        if let Some(consequences) = &choice.consequences {
                            for consequence in consequences {
                                match consequence {
                                    DialogConsequences::AddResource(resource_type, amount) => {
                                        player_datas.add_ore(*amount);
                                    },
                                    DialogConsequences::SubResource(resource_type, amount) => {
                                        player_datas.sub_ore(*amount);
                                    },
                                    DialogConsequences::SetResource(resource_type, amount) => {
                                        player_datas.set_ore(*amount);
                                    },
                                    DialogConsequences::SetSwitch(_, _) => todo!(),
                                }
                            }
                        }
                        if let Some(choice_event) = &choice.id_next {
                            let id_next = match choice_event {
                                Random((max, win), (win_id, lose_id)) => {
                                    let mut rng = rand::thread_rng();
                                    if rng.gen_range(0..*max) >= *win {
                                        win_id
                                    } else {
                                        lose_id
                                    }
                                },
                                Goto(id) => {
                                    id
                                }
                            };
                            ev.send(ReadNextDialog(Some(*id_next)));
                        } else {
                            ev.send(ReadNextDialog(None));
                        }
                        *color = bevy::prelude::UiColor(Color::rgb(0.85, 0.15, 0.15));
                    }
                }
            } 
            Interaction::Hovered => {
                *color = bevy::prelude::UiColor(Color::rgb(0.15, 0.85, 0.15));
            }
            Interaction::None => {
                *color = bevy::prelude::UiColor(Color::rgb(0.15, 0.15, 0.85));
            }
        }
    }
}




