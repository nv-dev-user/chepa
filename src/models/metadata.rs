struct Metadata {
    title: String,
    version: String,
    langage: String,
    author: Vec<String>,
    difficulties: Vec<f32>,
}

impl Metadata {
    fn new(title: String, version: String, langage: String, author: Vec<String>, difficulties: Vec<f32>) -> Self {
        Metadata { title, version, langage, author, difficulties }
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_langage(&self) -> &str {
        &self.langage
    }

    fn get_version(&self) -> &str {
        &self.version
    }

    fn get_author(&self) -> &[String] {
        &self.author
    }

    fn get_difficulties(&self) -> &[f32] {
        &self.difficulties
    }
}