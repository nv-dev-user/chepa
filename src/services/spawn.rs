use rand::RngExt;

use crate::models::npc::NPC;
use crate::models::spawn_group::SpawnGroup;
use crate::models::zone::Zone;

pub struct ZoneSpawnResult<'a> {
    pub spawned: bool,
    pub mob: Option<&'a NPC>,
}

pub fn try_spawn_after_zone_change<'a>(
    previous_zone_id: Option<u32>,
    current_zone: Option<&Zone>,
    spawn_groups: &[SpawnGroup],
    npcs: &'a [NPC],
) -> ZoneSpawnResult<'a> {
    let Some(zone) = current_zone else {
        return ZoneSpawnResult {
            spawned: false,
            mob: None,
        };
    };

    let current_zone_id = zone.get_entity().get_id();
    if previous_zone_id == Some(current_zone_id) {
        return ZoneSpawnResult {
            spawned: false,
            mob: None,
        };
    }

    try_spawn_on_zone_entry(zone, spawn_groups, npcs)
}

pub fn try_spawn_on_zone_entry<'a>(
    zone: &Zone,
    spawn_groups: &[SpawnGroup],
    npcs: &'a [NPC],
) -> ZoneSpawnResult<'a> {
    if !roll_spawn_rate(zone.get_spawn_rate()) {
        return ZoneSpawnResult {
            spawned: false,
            mob: None,
        };
    }

    let Some(spawn_group_id) = zone.get_spawn_group_id() else {
        return ZoneSpawnResult {
            spawned: false,
            mob: None,
        };
    };

    let Some(spawn_group) = search_spawn_group_by_id(spawn_group_id, spawn_groups) else {
        return ZoneSpawnResult {
            spawned: false,
            mob: None,
        };
    };

    let Some(npc) = pick_npc_from_spawn_group(spawn_group, npcs) else {
        return ZoneSpawnResult {
            spawned: false,
            mob: None,
        };
    };

    ZoneSpawnResult {
        spawned: true,
        mob: Some(npc),
    }
}

fn roll_spawn_rate(spawn_rate: u8) -> bool {
    if spawn_rate == 0 {
        return false;
    }

    let mut rng = rand::rng();
    let roll: u8 = rng.random_range(0..100);
    roll < spawn_rate
}

fn pick_npc_from_spawn_group<'a>(
    spawn_group: &SpawnGroup,
    npcs: &'a [NPC],
) -> Option<&'a NPC> {
    let weights = spawn_group.get_npcs();
    let total: f32 = weights.values().sum();

    if total <= 0.0 {
        return None;
    }

    let mut rng = rand::rng();
    let mut roll: f32 = rng.random_range(0.0..total);

    for (npc_id, weight) in weights {
        if roll < *weight {
            return search_npc_by_id(*npc_id, npcs);
        }
        roll -= weight;
    }

    None
}

pub fn search_spawn_group_by_id(id: u32, spawn_groups: &[SpawnGroup]) -> Option<&SpawnGroup> {
    spawn_groups
        .iter()
        .find(|spawn_group| spawn_group.get_id() == id)
}

pub fn search_npc_by_id(id: u32, npcs: &[NPC]) -> Option<&NPC> {
    npcs
        .iter()
        .find(|npc| npc.get_living_entity().get_entity().get_id() == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::models::entity::Entity;
    use crate::models::living_entity::LivingEntity;
    use crate::models::zone::Zone;

    fn make_zone(id: u32, spawn_rate: u8, spawn_group_id: Option<u32>) -> Zone {
        Zone::new(
            Entity::new(id, format!("Zone {id}")),
            1,
            spawn_rate,
            spawn_group_id,
            None,
            None,
            None,
            None,
        )
    }

    fn make_npc(id: u32, name: &str) -> NPC {
        let zone = make_zone(1, 100, Some(1));
        NPC::new(
            LivingEntity::new(
                Entity::new(id, name.to_string()),
                100,
                10,
                10,
                10,
                50,
                None,
                None,
                Some(zone.clone()),
                zone,
            ),
            vec![],
            HashMap::new(),
        )
    }

    fn make_spawn_group(id: u32, npcs: HashMap<u32, f32>) -> SpawnGroup {
        SpawnGroup::new(id, npcs)
    }

    #[test]
    fn no_spawn_when_zone_did_not_change() {
        let zone = make_zone(3, 100, Some(1));
        let spawn_groups = vec![make_spawn_group(1, HashMap::from([(1, 1.0)]))];
        let npcs = vec![make_npc(1, "Loup")];

        let result = try_spawn_after_zone_change(Some(3), Some(&zone), &spawn_groups, &npcs);

        assert!(!result.spawned);
        assert!(result.mob.is_none());
    }

    #[test]
    fn spawn_always_succeeds_with_100_percent_rate() {
        let zone = make_zone(3, 100, Some(1));
        let spawn_groups = vec![make_spawn_group(1, HashMap::from([(2, 1.0)]))];
        let npcs = vec![make_npc(2, "Sanglier")];

        let result = try_spawn_after_zone_change(Some(1), Some(&zone), &spawn_groups, &npcs);

        assert!(result.spawned);
        assert_eq!(
            result.mob.unwrap().get_living_entity().get_entity().get_id(),
            2
        );
    }

    #[test]
    fn spawn_picks_npc_from_spawn_group_weights() {
        let zone = make_zone(3, 100, Some(1));
        let spawn_groups = vec![make_spawn_group(
            1,
            HashMap::from([(1, 0.0), (3, 1.0)]),
        )];
        let npcs = vec![make_npc(1, "Loup"), make_npc(3, "Tigre")];

        let result = try_spawn_on_zone_entry(&zone, &spawn_groups, &npcs);

        assert!(result.spawned);
        assert_eq!(
            result.mob.unwrap().get_living_entity().get_entity().get_id(),
            3
        );
    }

    #[test]
    fn no_spawn_without_spawn_group() {
        let zone = make_zone(3, 100, None);
        let spawn_groups = vec![make_spawn_group(1, HashMap::from([(1, 1.0)]))];
        let npcs = vec![make_npc(1, "Loup")];

        let result = try_spawn_on_zone_entry(&zone, &spawn_groups, &npcs);

        assert!(!result.spawned);
        assert!(result.mob.is_none());
    }
}
