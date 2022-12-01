use std::{fs::{File, self}, io::Write};

use bevy::{utils::HashMap, prelude::ResMut};

use crate::resources::{dialog::{DialogDatas, DialogData, DialogChoice, DialogChoiceEvent, DialogConsequences, DialogCondition}, resources_enums::InGameResourceType};

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
            consequences: None,
            id_next: Some(DialogChoiceEvent::Goto(2)),
        }],    
    });

    datas.dialogs.insert(2, DialogData {
        text: String::from("I need ore, can you give me 20 please ?"),
        choices: vec![DialogChoice {
            text: String::from("Sure, take this my friend"),
            conditions: Some(vec!(DialogCondition::ResourceCheck(InGameResourceType::Ore, 20))),
            consequences: Some(vec!(DialogConsequences::SubResource(InGameResourceType::Ore, 20))),
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
            consequences: Some(vec!(DialogConsequences::SubResource(InGameResourceType::Ore, 1))),
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