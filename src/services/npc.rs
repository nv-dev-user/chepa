use crate::models::npc::NPC;
use crate::models::zone::Zone;

pub fn get_npc_by_zone<'a>(zone: Option<&Zone>, npcs: &'a mut [NPC]) -> Vec<&'a mut NPC> {
    let mut npcs_in_zone = Vec::new();
    let zone = match zone {
        Some(z) => z,
        None => return npcs_in_zone,
    };
    for npc in npcs.iter_mut() {
        if let Some(npc_zone) = npc.get_living_entity().get_zone() {
            if npc_zone.get_entity().get_id() == zone.get_entity().get_id() {
                npcs_in_zone.push(npc);
            }
        }
    }
    npcs_in_zone
}