#[derive(Debug, PartialEq)]
pub struct Connection {
    pub host: String,
    pub username: String,
    pub password: String,
    pub download_dir: String,
    pub regex: String,
    pub port: u32,
    pub provider: String,
}
