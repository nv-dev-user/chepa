
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::models::armor::Armor;
use crate::models::metadata::Metadata;
use crate::models::npc::NPC;
use crate::models::player::Player;
use crate::models::weapon::Weapon;
use crate::models::zone::Zone;

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

pub fn load_zones(content : &str) -> io::Result<Vec<Zone>>{
    let zones: Vec<Zone> = serde_json::from_str(content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(zones)
}

pub fn load_armors(content : &str) -> io::Result<Vec<Armor>> {
    let armors: Vec<Armor> = serde_json::from_str(content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(armors)
}

pub fn load_metadata(content : &str) -> io::Result<Metadata> {
    let metadata: Metadata = serde_json::from_str(content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(metadata)
}

pub fn load_weapons(content : &str) -> io::Result<Vec<Weapon>>{
    let weapons: Vec<Weapon> = serde_json::from_str(content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(weapons)
}

pub fn load_npc(content : &str) -> io::Result<Vec<NPC>>{
    let npcs: Vec<NPC> = serde_json::from_str(content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(npcs)
}

pub fn load_player(content : &str) -> io::Result<Player>{
    let player: Player = serde_json::from_str(content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(player)
}

#[cfg(test)]
mod tests {
    use jsonparser::JSONParser;

    use crate::models::entity::Entity;

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
        [
            {
                "entity": {
                    "id": 1,
                    "name": "Zone 1"
                },
                "base_level": 1,
                "spawn_rate": 10,
                "spawn_group_id": null,
                "north_zone_id": 2,
                "east_zone_id": null,
                "south_zone_id": null,
                "west_zone_id": null
            },
            {
                "entity": {
                    "id": 2,
                    "name": "Zone 2"
                },
                "base_level": 1,
                "spawn_rate": 10,
                "spawn_group_id": null,
                "north_zone_id": null,
                "east_zone_id": null,
                "south_zone_id": null,
                "west_zone_id": 1
            }
        ]
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
    fn test_parse_json() {
        let content = r#"
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
        }
        "#;

        let object: serde_json::Value = serde_json::from_str(content).unwrap();

        let id = object["id"].as_u64().unwrap() as u32;
        let name = object["name"].as_str().unwrap().to_string();
        let base_level = object["base_level"].as_u64().unwrap() as u32;
        let spawn_rate = object["spawn_rate"].as_u64().unwrap() as u8;
        let north_zone_id = object["north_zone_id"].as_u64().map(|n| n as u32);

        let zone = Zone::new(
            Entity::new(id, name),
            base_level,
            spawn_rate,
            None,
            north_zone_id,
            None,
            None,
            None
        );

        //println!("Object: {:?}", object);
        println!("Zone: {:?}", zone);
    }
}
