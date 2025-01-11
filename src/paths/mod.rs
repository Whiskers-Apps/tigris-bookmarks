use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    PathBuf::from(dirs::config_dir().unwrap()).join("tigris-bookmarks")
}

pub fn get_db_path() -> PathBuf {
    get_config_dir().join("db.json")
}

pub fn get_favicons_dir() -> PathBuf {
    get_config_dir().join("favicons")
}

pub fn get_favicon_path(name: &str) -> PathBuf {
    get_favicons_dir().join(format!("{name}.png"))
}
