use std::{fs::{File, self}, io::Write};

use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{resources::{dialog::{DialogDatas, CurrentDialog, DialogData, DialogChoice, DialogChoiceEvent, DialogConsequences, DialogCondition}, resources_enums::InGameResourceType}, DialogBox, DialogText};
use crate::plugins::dialog::DialogChoiceEvent::*;
use rand::prelude::*;

pub struct ReadNextDialog(Option<u32>);

#[derive(Component)]
pub struct DialogChoiceButton {
    pub id: usize
}

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
    mut dialog_box: Query<&mut Visibility, With<DialogBox>>,
    mut dialog_text: Query<&mut Text, With<DialogText>>
) {
    for mut visibility in dialog_box.iter_mut() {
        for mut text in dialog_text.iter_mut() {
            if let Some(dialog) = &current.dialog {
                visibility.is_visible = true;
                text.sections[0].value = format!("{}", dialog.text);
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
    mut ev: EventWriter<ReadNextDialog>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(dialog) = &current.dialog {
                    if let Some(choice) = dialog.choices.get(button.id) {
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
                        *color = bevy::prelude::UiColor(Color::rgb(0.85, 0.15, 0.15))
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




// Utility functions
pub fn create_json (
) {
    let mut datas = DialogDatas {
        dialogs: HashMap::new(),
    };

    datas.dialogs.insert(0, DialogData {
        text: String::from("Here's a bunch of dialogs for testing purposes"),
        choices: vec![DialogChoice {
            text: String::from("Uh... Yeah, sure !"),
            conditions: None,
            consequences: None,
            id_next: Some(DialogChoiceEvent::Goto(1)),
        }],
    });

    datas.dialogs.insert(1, DialogData {
        text: String::from("Do you want some free ore ?"),
        choices: vec![DialogChoice {
            text: String::from("Sure !"),
            conditions: None,
            consequences: Some(vec!(DialogConsequences::AddResource(InGameResourceType::Ore, 25))),
            id_next: Some(DialogChoiceEvent::Goto(2)),
        },
        DialogChoice {
            text: String::from("No thanks, it sounds like a trap"),
            conditions: None,
            consequences: Some(vec!(DialogConsequences::AddResource(InGameResourceType::Ore, -10))),
            id_next: Some(DialogChoiceEvent::Goto(2)),
        }],    
    });

    datas.dialogs.insert(2, DialogData {
        text: String::from("I need ore, can you give me 20 please ?"),
        choices: vec![DialogChoice {
            text: String::from("Sure, take this my friend"),
            conditions: Some(vec!(DialogCondition::ResourceCheck(InGameResourceType::Ore, 20))),
            consequences: Some(vec!(DialogConsequences::AddResource(InGameResourceType::Ore, 20))),
            id_next: Some(DialogChoiceEvent::Goto(3)),
        },
        DialogChoice {
            text: String::from("Nope"),
            conditions: None,
            consequences: None,
            id_next: Some(DialogChoiceEvent::Goto(3)),
        }],  
    });

    datas.dialogs.insert(3, DialogData {
        text: String::from("Wanna play a coin toss game with me ? The bet will be 1 ore"),
        choices: vec![DialogChoice {
            text: String::from("I'm in !"),
            conditions: Some(vec!(DialogCondition::ResourceCheck(InGameResourceType::Ore, 1))),
            consequences: Some(vec!(DialogConsequences::AddResource(InGameResourceType::Ore, -1))),
            id_next: Some(DialogChoiceEvent::Random((100, 50),(4, 5))),
        },
        DialogChoice {
            text: String::from("I have better things to do, leave me alone"),
            conditions: None,
            consequences: None,
            id_next: Some(DialogChoiceEvent::Goto(6)),
        }],  
    });

    datas.dialogs.insert(4, DialogData {
        text: String::from("GG, you won ! Here's your reward"),
        choices: vec![DialogChoice {
            text: String::from("Thanks ! See ya"),
            conditions: None,
            consequences: Some(vec!(DialogConsequences::AddResource(InGameResourceType::Ore, 2))),
            id_next: None,
        }], 
    });

    datas.dialogs.insert(5, DialogData {
        text: String::from("Heh, I won !"),
        choices: vec![DialogChoice {
            text: String::from("GG WP"),
            conditions: None,
            consequences: None,
            id_next: None,
        }], 
    });

    datas.dialogs.insert(6, DialogData {
        text: String::from("*sad*"),
        choices: vec![DialogChoice {
            text: String::from("I don't care"),
            conditions: None,
            consequences: None,
            id_next: None,
        }], 
    });

    let j = serde_json::to_string_pretty(&datas).unwrap();

    let mut file = File::create("dialogs.json").unwrap();
    file.write_all(j.as_bytes()).unwrap();
}

pub fn load_json (
    mut datas: ResMut<DialogDatas>,
) {
    let file = fs::read_to_string("dialogs.json").unwrap();
    *datas = serde_json::from_str(&file).unwrap();
}