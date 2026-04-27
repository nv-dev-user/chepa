
use std::fs;
use std::io;
use std::path::PathBuf;

use jsonparser;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

use crate::models::zone::Zone;
use crate::models::weapon::Weapon;


/*
* Recherche les fichiers de données dans le répertoire ./src/data
* Retourne une liste de fichier JSON
*/
fn search_datafiles() -> std::io::Result<Vec<PathBuf>> {
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
    let parsed_content: Vec<Zone> = serde_json::from_str(content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(parsed_content)
}

pub fn load_weapons(content : &str) -> io::Result<Vec<Weapon>>{
    let parsed_content: Vec<Weapon> = serde_json::from_str(content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(parsed_content)
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
            assert!(file.extension().unwrap() == "json");
        }
    }
}
