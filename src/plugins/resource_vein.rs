use bevy::prelude::*;
use bevy_text_mesh::TextMesh;

use crate::{components::{datas::PlayerData, resource_vein::ResourceVeinComponent, can_interact::CanInteract}, resources::interact_state::InteractState, InteractionStateEvent};

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

            let workers = vein.workers.len() as u32;
            let n = if vein.amount >= workers {
                workers
            } else {
                vein.amount
            };
            player_data.ore += n as u32;
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