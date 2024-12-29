pub struct Config {
    pub db_url: String,
}

pub fn load() -> Config {
    Config {
        db_url: "".to_string(),
    }
}
