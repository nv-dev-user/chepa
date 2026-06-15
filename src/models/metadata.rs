#[derive(Debug, serde::Deserialize)]
pub struct Metadata {
    title: String,
    version: String,
    langage: String,
    author: Vec<String>,
    difficulties: Vec<f32>,
}

impl Metadata {
    pub fn new(title: String, version: String, langage: String, author: Vec<String>, difficulties: Vec<f32>) -> Self {
        Metadata { title, version, langage, author, difficulties }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_langage(&self) -> &str {
        &self.langage
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_author(&self) -> &[String] {
        &self.author
    }

    pub fn get_difficulties(&self) -> &[f32] {
        &self.difficulties
    }
}