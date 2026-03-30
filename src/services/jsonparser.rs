use jsonparser;
use std::fs;
use std::path::PathBuf;

/*
* Recherche les fichiers de données dans le répertoire ./src/data
* Retourne une liste de fichier JSON
*/
fn search_datafiles() -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir("./src/data")? {
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
* CA MARCHE PAS -> askip jsonparser c'est nul
* C'est sensé concaténer les fichiers JSON
*/
pub fn initialize_data() -> std::io::Result<()> {
    let files = search_datafiles();

    let mut merged = JSONValue::Object(Default::default());

    for file in files {
        println!("Traitement : {:?}", file);

        let content = match read_file_to_string(&file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Erreur lecture {} : {:?}", file.display(), e);
                continue;
            }
        };

        let json_val = match parse_json_string(&content) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Erreur parse JSON dans {} : {}", file.display(), e);
                continue;
            }
        };

        merged.merge(&json_val);
    }

    println!("JSON concaténé : {:#?}", merged);

    Ok(())

}

