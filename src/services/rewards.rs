use rand::RngExt;
use crate::models::item::Item;
use crate::models::living_entity::LivingEntity;
use crate::models::npc::NPC;
use crate::models::player::Player;

pub fn compute_exp_gain(_player: &Player, entity: &LivingEntity) -> u32 {
    let base_xp = entity.get_base_xp() as f32;
    (base_xp * 0.4).round().max(1.0) as u32
}

pub fn random_drop(npc: &NPC) -> Option<&Item> {
    let map = npc.get_droppable_items_percentage();
    let total: f32 = map.values().sum();

    if total <= 0.0 {
        return None;
    }

    let mut rng = rand::rng();
    let mut roll: f32 = rng.random_range(0.0..total);

    for (item, weight) in map {
        if roll < *weight {
            return Some(item);
        }
        roll -= weight;
    }

    None
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::models::entity::Entity;
    use crate::models::living_entity::LivingEntity;
    use crate::models::npc::NPC;
    use crate::models::player::Player;
    use crate::models::zone::Zone;

    fn make_zone() -> Zone {
        Zone::new(
            Entity::new(1, "Plaine".to_string()),
            1,
            50,
            None,
            None,
            None,
            None,
            None,
        )
    }

    fn make_living_entity(id: u32, name: &str, base_xp: u32) -> LivingEntity {
        LivingEntity::new(
            Entity::new(id, name.to_string()),
            100,
            10,
            10,
            10,
            base_xp,
            None,
            None,
            Some(make_zone().clone()),
            make_zone().clone(),
        )
    }

    #[test]
    fn compute_exp_gain_returns_40_percent_of_base_xp() {
        let player = Player::new(make_living_entity(10, "Player", 0), 0);
        let npc_entity = make_living_entity(20, "Loup", 100);

        let gained = compute_exp_gain(&player, &npc_entity);

        assert_eq!(gained, 40);
    }

    #[test]
    fn compute_exp_gain_has_minimum_one() {
        let player = Player::new(make_living_entity(11, "Player", 0), 0);
        let npc_entity = make_living_entity(21, "Rat", 1);

        let gained = compute_exp_gain(&player, &npc_entity);

        assert_eq!(gained, 1);
    }

    #[test]
    fn random_drop_returns_none_when_no_drops_defined() {
        let npc = NPC::new(
            make_living_entity(30, "Bandit", 50),
            vec!["...".to_string()],
            HashMap::new(),
        );

        let drop = random_drop(&npc);

        assert!(drop.is_none());
    }

    #[test]
    fn random_drop_picks_100_percent_item_over_0_percent_item() {
        let mut drops = HashMap::new();
        drops.insert(Item::new(100, "Drop garanti".to_string()), 100.0);
        drops.insert(Item::new(101, "Drop impossible".to_string()), 0.0);

        let npc = NPC::new(
            make_living_entity(31, "Loup", 50),
            vec!["Grr".to_string()],
            drops,
        );

        let drop = random_drop(&npc);

        assert!(drop.is_some());
        assert_eq!(drop.unwrap().get_name(), "Drop garanti");
    }
}
