
use std::fs;
use std::io;
use std::path::PathBuf;

use jsonparser::JSONParser;

use crate::models::armor::Armor;
use crate::models::entity::Entity;
use crate::models::metadata::Metadata;
use crate::models::npc::NPC;
use crate::models::player::Player;
use crate::models::weapon::Weapon;
use crate::models::zone::Zone;
use crate::models::item::Item;
use crate::models::living_entity::LivingEntity;
use crate::services::zones::searchZoneById;
use std::collections::HashMap;

/*
* Recherche les fichiers de données dans le répertoire ./src/data
* Retourne une liste de fichier JSON
*/
pub fn search_datafiles() -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir("./data")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "json" {
                    files.push(path);
                }
            }
        }
    }

    if files.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Aucun fichier JSON trouvé",
        ));
    }
    else {
        Ok(files)
    }
}

/*
 * Lit le contenu d'un fichier JSON et le retourne sous forme de String
 */
pub fn receive_data_from_file(path : &str) -> io::Result<String> {
    let json_content = fs::read_to_string(path)?;

    if json_content.is_empty() {
        return Err(
            io::Error::new(
                io::ErrorKind::NotFound,
                "Aucun contenu dans le fichier JSON trouvé",
            )
        );
    }
    Ok(json_content)
}

pub fn load_zones(content : &str) -> io::Result<Vec<Zone>> {

    let result = JSONParser::new(content).parse();

    let zones_result = match result {
        Ok(z) => z,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
    };

    let zones_obj = zones_result.as_object()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected object"))?;

    let zones_array = zones_obj.get("zones")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing zones key"))?
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected array"))?;

    let mut zone_list = Vec::new();

    for z in zones_array {
        let id = z["id"].as_f64().unwrap() as u32;
        let name = z["name"].as_str().unwrap().to_string();
        let base_level = z["base_level"].as_f64().unwrap() as u32;
        let spawn_rate = z["spawn_rate"].as_f64().unwrap() as u8;
        let spawn_group_id = z["spawn_group_id"].as_f64().map(|v| v as u32);
        let north_zone_id = z["north_zone_id"].as_f64().map(|v| v as u32);
        let east_zone_id = z["east_zone_id"].as_f64().map(|v| v as u32);
        let south_zone_id = z["south_zone_id"].as_f64().map(|v| v as u32);
        let west_zone_id = z["west_zone_id"].as_f64().map(|v| v as u32);

        zone_list.push(Zone::new(Entity::new(id, name), base_level, spawn_rate, spawn_group_id, north_zone_id, east_zone_id, south_zone_id, west_zone_id));
    }

    Ok(zone_list)
}

pub fn load_armors(content : &str) -> io::Result<Vec<Armor>> {

    let result = JSONParser::new(content).parse();

    let armors_result = match result {
        Ok(z) => z,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
    };

    let armors_obj = armors_result.as_object()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected object"))?;

    let armors_array = armors_obj.get("armors")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing armors key"))?
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected array"))?;

    let mut armor_list = Vec::new();

    for a in armors_array {
        let id = a["id"].as_f64().unwrap() as u32;
        let name = a["name"].as_str().unwrap().to_string();
        let coef_modifier = a["coef_modifier"].as_f64().unwrap() as u32;

        armor_list.push(Armor::new(id, name, coef_modifier));
    }

    Ok(armor_list)
}

pub fn load_metadata(content : &str) -> io::Result<Metadata> {
    let result = JSONParser::new(content).parse();

    let metadata_obj = match result {
        Ok(z) => z,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
    };

    let obj = metadata_obj.as_object()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected object"))?;

    let title = obj.get("title")
        .and_then(|v| v.as_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid title"))?
        .to_string();

    let version = obj.get("version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid version"))?
        .to_string();

    let langage = obj.get("langage")
        .and_then(|v| v.as_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid langage"))?
        .to_string();

    let author = obj.get("author")
        .and_then(|v| v.as_array())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid author"))?
        .iter()
        .map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<Option<Vec<String>>>()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid author entries"))?;

    let difficulties = obj.get("difficulties")
        .and_then(|v| v.as_array())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid difficulties"))?
        .iter()
        .map(|v| v.as_f64())
        .collect::<Option<Vec<f64>>>()
        .map(|v| v.iter().map(|&f| f as f32).collect())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid difficulties entries"))?;

    Ok(Metadata::new(title, version, langage, author, difficulties))
}

pub fn load_weapons(content : &str) -> io::Result<Vec<Weapon>>{

    let result = JSONParser::new(content).parse();

    let weapons_result = match result {
        Ok(z) => z,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
    };

    let weapons_obj = weapons_result.as_object()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected object"))?;

    let weapons_array = weapons_obj.get("weapons")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing weapons key"))?
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected array"))?;

    let mut weapon_list = Vec::new();

    for w in weapons_array {
        let id = w["id"].as_f64().unwrap() as u32;
        let name = w["name"].as_str().unwrap().to_string();
        let coef_modifier = w["coef_modifier"].as_f64().unwrap() as u32;

        weapon_list.push(Weapon::new(id, name, coef_modifier));
    }

    Ok(weapon_list)
}

pub fn load_npc(content : &str, zones: Vec<Zone>) -> io::Result<Vec<NPC>>{

    let result = JSONParser::new(content).parse();

    let npcs_result = match result {
        Ok(z) => z,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
    };

    let npcs_obj = npcs_result.as_object()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected object"))?;

    let npcs_array = npcs_obj.get("npcs")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing npcs key"))?
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected array"))?;

    let mut npc_list = Vec::new();

    for npc_data in npcs_array {
        let id = npc_data["id"].as_f64().unwrap() as u32;
        let name = npc_data["name"].as_str().unwrap().to_string();
        let zone_id = npc_data["zone_id"].as_f64().unwrap() as u32;

        let base_hp = npc_data["base_hp"].as_f64().unwrap() as u32;
        let base_attack = npc_data["base_attack"].as_f64().unwrap() as u32;
        let base_speed = npc_data["base_speed"].as_f64().unwrap() as u32;
        let base_parade = npc_data["base_parade"].as_f64().unwrap() as u32;
        let base_xp = npc_data["base_xp"].as_f64().unwrap() as u32;

        let zone = searchZoneById(zone_id, &zones)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, format!("Zone {} not found", zone_id)))?
            .clone();

        let entity = Entity::new(id, name);
        let living_entity = LivingEntity::new(entity, base_hp, base_attack, base_speed, base_parade, base_xp, None, None, zone.clone(), zone);

        let dialogs: Vec<String> = npc_data["dialogs"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|d| d.as_str().map(|s| s.to_string()))
            .collect();

        let droppable_items_percentage: HashMap<Item, f32> = HashMap::new(); // Pour simplifier

        npc_list.push(NPC::new(living_entity, dialogs, droppable_items_percentage));
    }

    Ok(npc_list)
}

pub fn load_player(content : &str, zones: Vec<Zone>) -> io::Result<Player>{
    let result = JSONParser::new(content).parse();

    let player_obj = match result {
        Ok(z) => z,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
    };

    let obj = player_obj.as_object()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected object"))?;

    let id = obj.get("id")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid id"))?
        as u32;

    let name = obj.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid name"))?
        .to_string();

    let zone_id = obj.get("zone_id")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid zone_id"))?
        as u32;

    let base_hp = obj.get("base_hp")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid base_hp"))?
        as u32;

    let base_attack = obj.get("base_attack")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid base_attack"))?
        as u32;

    let base_speed = obj.get("base_speed")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid base_speed"))?
        as u32;

    let base_parade = obj.get("base_parade")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid base_parade"))?
        as u32;

    let base_xp = obj.get("base_xp")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid base_xp"))?
        as u32;

    let xp_total = obj.get("xp_total")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid xp_total"))?
        as u64;

    let zone = searchZoneById(zone_id, &zones)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, format!("Zone {} not found", zone_id)))?
        .clone();

    let entity = Entity::new(id, name);
    let living_entity = LivingEntity::new(entity, base_hp, base_attack, base_speed, base_parade, base_xp, None, None, zone.clone(), zone);

    Ok(Player::new(living_entity, xp_total))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_datafiles() {
        let result = search_datafiles();
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(!files.is_empty());
        for file in files {
            println!("Fichier trouvé : {:?}", file);
            assert!(file.extension().unwrap() == "json");
        }
    }

    #[test]
    fn test_receive_data_from_file() {
        let result = receive_data_from_file("./data/zones.json");
        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(!content.is_empty());
        println!("Contenu du fichier : {}", content);
    }

    #[test]
    fn test_load_zones() {
        let content = r#"
        {
            "zones": [
                {
                    "id": 1,
                    "name": "Zone 1",
                    "base_level": 1,
                    "spawn_rate": 10,
                    "spawn_group_id": null,
                    "north_zone_id": 2,
                    "east_zone_id": null,
                    "south_zone_id": null,
                    "west_zone_id": null
                },
                {
                    "id": 2,
                    "name": "Zone 2",
                    "base_level": 1,
                    "spawn_rate": 10,
                    "spawn_group_id": null,
                    "north_zone_id": null,
                    "east_zone_id": null,
                    "south_zone_id": null,
                    "west_zone_id": 1
                }
            ]
        }
        "#;

        let result = load_zones(content);
        assert!(result.is_ok());
        let zones = result.unwrap();
        assert_eq!(zones.len(), 2);
        assert_eq!(zones[0].get_entity().get_id(), 1);
        assert_eq!(zones[0].get_entity().get_name(), "Zone 1");
        assert_eq!(zones[0].get_north_zone_id(), Some(2));
        assert_eq!(zones[0].get_east_zone_id(), None);
        assert_eq!(zones[0].get_south_zone_id(), None);
        assert_eq!(zones[0].get_west_zone_id(), None);
        assert_eq!(zones[1].get_entity().get_id(), 2);
        assert_eq!(zones[1].get_entity().get_name(), "Zone 2");
        assert_eq!(zones[1].get_north_zone_id(), None);
        assert_eq!(zones[1].get_east_zone_id(), None);
        assert_eq!(zones[1].get_south_zone_id(), None);
        assert_eq!(zones[1].get_west_zone_id(), Some(1));
    }

    #[test]
    fn test_load_armors() {
        let content = r#"
        {
            "armors": [
                {
                    "id": 1,
                    "name": "Tunique usee",
                    "coef_modifier": 6
                },
                {
                    "id": 2,
                    "name": "Armure legere",
                    "coef_modifier": 8
                }
            ]
        }
        "#;

        let result = load_armors(content);
        assert!(result.is_ok());
        let armors = result.unwrap();
        assert_eq!(armors.len(), 2);
        assert_eq!(armors[0].get_id(), 1);
        assert_eq!(armors[0].get_name(), "Tunique usee");
        assert_eq!(armors[0].get_coef_modifier(), 6);
        assert_eq!(armors[1].get_id(), 2);
        assert_eq!(armors[1].get_name(), "Armure legere");
        assert_eq!(armors[1].get_coef_modifier(), 8);
    }

    #[test]
    fn test_load_weapons() {
        let content = r#"
        {
            "weapons": [
                {
                    "id": 1,
                    "name": "Dague en fer",
                    "coef_modifier": 8
                },
                {
                    "id": 2,
                    "name": "Épée en acier",
                    "coef_modifier": 12
                }
            ]
        }
        "#;

        let result = load_weapons(content);
        assert!(result.is_ok());
        let weapons = result.unwrap();
        assert_eq!(weapons.len(), 2);
        assert_eq!(weapons[0].get_id(), 1);
        assert_eq!(weapons[0].get_name(), "Dague en fer");
        assert_eq!(weapons[0].get_coef_modifier(), 8);
        assert_eq!(weapons[1].get_id(), 2);
        assert_eq!(weapons[1].get_name(), "Épée en acier");
        assert_eq!(weapons[1].get_coef_modifier(), 12);
    }

    #[test]
    fn test_load_metadata() {
        let content = r#"
        {
            "title": "Chepa Game",
            "version": "0.1.0",
            "langage": "Rust",
            "author": ["Alice", "Bob"],
            "difficulties": [1.0, 2.5, 5.0]
        }
        "#;

        let result = load_metadata(content);
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.get_title(), "Chepa Game");
        assert_eq!(metadata.get_version(), "0.1.0");
        assert_eq!(metadata.get_langage(), "Rust");
        assert_eq!(metadata.get_author().len(), 2);
        assert_eq!(metadata.get_author()[0], "Alice");
        assert_eq!(metadata.get_author()[1], "Bob");
        assert_eq!(metadata.get_difficulties().len(), 3);
        assert_eq!(metadata.get_difficulties()[0], 1.0);
        assert_eq!(metadata.get_difficulties()[1], 2.5);
        assert_eq!(metadata.get_difficulties()[2], 5.0);
    }

    #[test]
    fn test_load_npc() {
        use crate::models::entity::Entity;

        // Créer les zones pour le test
        let zone_entity = Entity::new(1, "Zone 1".to_string());
        let zone = Zone::new(zone_entity, 1, 10, None, None, None, None, None);
        let zones = vec![zone];

        let content = r#"
        {
            "npcs": [
                {
                    "id": 1,
                    "name": "Loup gris",
                    "zone_id": 1,
                    "base_hp": 85,
                    "base_attack": 18,
                    "base_speed": 22,
                    "base_parade": 10,
                    "base_xp": 70,
                    "dialogs": ["Grrrr...", "Awooooo"]
                }
            ]
        }
        "#;

        let result = load_npc(content, zones.clone());
        assert!(result.is_ok());
        let npcs = result.unwrap();
        assert_eq!(npcs.len(), 1);
        assert_eq!(npcs[0].get_living_entity().get_entity().get_id(), 1);
        assert_eq!(npcs[0].get_living_entity().get_entity().get_name(), "Loup gris");
        assert_eq!(npcs[0].get_living_entity().get_base_hp(), 85);
        assert_eq!(npcs[0].get_living_entity().get_base_attack(), 18);
        assert_eq!(npcs[0].get_dialogs().len(), 2);
        assert_eq!(npcs[0].get_dialogs()[0], "Grrrr...");
    }

    #[test]
    fn test_load_player() {
        use crate::models::entity::Entity;

        // Créer les zones pour le test
        let zone_entity = Entity::new(1, "Zone 1".to_string());
        let zone = Zone::new(zone_entity, 1, 10, None, None, None, None, None);
        let zones = vec![zone];

        let content = r#"
        {
            "id": 1,
            "name": "Hero",
            "zone_id": 1,
            "base_hp": 100,
            "base_attack": 15,
            "base_speed": 12,
            "base_parade": 8,
            "base_xp": 50,
            "xp_total": 150
        }
        "#;

        let result = load_player(content, zones);
        assert!(result.is_ok());
        let player = result.unwrap();
        assert_eq!(player.get_living_entity().get_entity().get_id(), 1);
        assert_eq!(player.get_living_entity().get_entity().get_name(), "Hero");
        assert_eq!(player.get_living_entity().get_base_hp(), 100);
        assert_eq!(player.get_living_entity().get_base_attack(), 15);
        assert_eq!(player.get_xp_total(), 150);
    }
}
