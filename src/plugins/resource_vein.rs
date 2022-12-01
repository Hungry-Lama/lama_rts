use bevy::prelude::*;
use bevy_text_mesh::TextMesh;

use crate::{components::{resource_vein::ResourceVeinComponent, can_interact::CanInteract}, resources::{interact_state::{InteractState, InteractionStateEvent}, techs_enums::Techs, player::data::PlayerData}};

pub struct ResourceVeinPlugin;

impl Plugin for ResourceVeinPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(collect_resource)
        .add_system(cleanup_empty_resource_vein)
        .add_system(update_resource_vein_remaining_text);
    }
}

pub fn collect_resource (
    mut player_data: ResMut<PlayerData>,
    mut q_veins: Query<&mut ResourceVeinComponent>,
    q_workers: Query<&CanInteract>,
    time: Res<Time>,
) {
    for mut vein in q_veins.iter_mut() {

        vein.timer.tick(time.delta());

        if vein.timer.finished() {
            vein.workers.retain(|worker| {
                let delete = {
                    if let Ok(w) = q_workers.get(*worker) {
                        match w.state {
                            InteractState::Interacting => false,
                            _ => true,
                        }
                    } else {
                        true
                    }
                };
                !delete
            });

            let mut workers = vein.workers.len() as u32;

            if let Some(t) = player_data.techs.get(&Techs::BetterOreMining) {
                if *t == true {
                    workers *= 2;
                }
            }

            let n = if vein.amount >= workers {
                workers
            } else {
                vein.amount
            };
            player_data.add_ore(n);
            vein.amount -= n as u32;
        }
    }
}

pub fn cleanup_empty_resource_vein (
    mut commands: Commands,
    q_veins: Query<(Entity, &ResourceVeinComponent)>,
    mut ev_interaction_workers: EventWriter<InteractionStateEvent>,
) {
    for (entity, vein) in q_veins.iter() {
        if vein.amount <= 0 {
            for worker in &vein.workers {
                ev_interaction_workers
                .send(InteractionStateEvent(*worker, InteractState::StandBy));
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn update_resource_vein_remaining_text(
    q_parent: Query<(&ResourceVeinComponent, &Children)>,
    mut q_child: Query<&mut TextMesh>,
) {
    for (vein, children) in q_parent.iter() {
        for &child in children {
            if let Ok(mut text) = q_child.get_mut(child) {
                text.text = format!("Remaining ore : {}", vein.amount);
            }
        }
    }
}