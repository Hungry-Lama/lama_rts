use bevy::prelude::*;

use crate::{components::{datas::PlayerData, resource_vein::ResourceVeinComponent, can_interact::CanInteract}, resources::interact_state::InteractState};

pub fn collect_resource (
    mut player_data: ResMut<PlayerData>,
    mut q_veins: Query<&mut ResourceVeinComponent>,
    q_workers: Query<&CanInteract>,
    time: Res<Time>,
) {
    for (mut vein) in q_veins.iter_mut() {

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
        
        vein.timer.tick(time.delta());

        if vein.timer.finished() {
            player_data.ore += vein.workers.len() as u32;
            vein.amount -= vein.workers.len() as u32;
        }
    }
}