use std::{fs::{File, self}, io::Write};

use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{components::datas::{DialogDatas, CurrentDialog, DialogData}, DialogBox, DialogText};

pub struct NextDialogButtonClicked;

pub fn read_dialog (
    datas: Res<DialogDatas>,
    mut current: ResMut<CurrentDialog>,
    mut events: EventReader<NextDialogButtonClicked>,
) {
    for _ev in events.iter() {
        if let Some(dialog) = &current.dialog {
            if let Some(next) = dialog.id_next {
                current.dialog = datas.dialogs.get(&next).cloned();
            } else {
                current.dialog = None;
            }
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

pub fn button_next_dialog(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut current: ResMut<CurrentDialog>,
    mut ev: EventWriter<NextDialogButtonClicked>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                ev.send(NextDialogButtonClicked);
                *color = bevy::prelude::UiColor(Color::rgb(0.85, 0.15, 0.15))
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
        text: String::from("Premier dialogue"),
        id_next: Some(1),
    });

    datas.dialogs.insert(1, DialogData {
        text: String::from("Second dialogue"),
        id_next: Some(2),
    });

    datas.dialogs.insert(2, DialogData {
        text: String::from("Troisieme et dernier dialogue"),
        id_next: None,
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