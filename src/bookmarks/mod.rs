use std::fs;

use serde::{Deserialize, Serialize};

use crate::paths::{get_config_dir, get_db_path};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookmarksDB {
    pub bookmarks: Vec<Bookmark>,
    pub groups: Vec<Group>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Bookmark {
    pub id: usize,
    pub name: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Group {
    pub id: usize,
    pub name: String,
    pub bookmarks_ids: Vec<usize>,
}

pub fn get_db() -> BookmarksDB {
    let config_dir = get_config_dir();
    let db_path = get_db_path();

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Error creating bookmarks directory");
    }

    if !db_path.exists() {
        let db = BookmarksDB {
            bookmarks: vec![],
            groups: vec![],
        };

        let json = serde_json::to_string(&db).expect("Error serializing bookmarks");

        fs::write(&db_path, &json).expect("Error writing db");

        return db;
    }

    let db_json = fs::read_to_string(&db_path).expect("Error reading db");
    let mut db = serde_json::from_str::<BookmarksDB>(&db_json).expect("Error deserializing db");

    db.bookmarks.sort_by_key(|bookmark| bookmark.name.clone());
    db.groups.sort_by_key(|group| group.name.clone());

    db
}

pub fn write_db(db: &BookmarksDB) {
    let json = serde_json::to_string(db).expect("Error serializing db");
    fs::write(&get_db_path(), &json).expect("Error writing db");
}

impl Bookmark {
    pub fn new(name: &str, link: &str) -> Self {
        let db = get_db();
        let id: usize = if db.bookmarks.is_empty() {
            0
        } else {
            let max = db
                .bookmarks
                .iter()
                .map(|bookmark| bookmark.id)
                .max()
                .unwrap();

            max + 1
        };

        Self {
            id,
            name: name.to_owned(),
            link: link.to_owned(),
        }
    }
}

impl Group {
    pub fn new(name: &str, bookmarks_ids: &Vec<usize>) -> Self {
        let db = get_db();
        let id: usize = if db.groups.is_empty() {
            0
        } else {
            let max = db.groups.iter().map(|group| group.id).max().unwrap();

            max + 1
        };

        Self {
            id,
            name: name.to_owned(),
            bookmarks_ids: bookmarks_ids.to_owned(),
        }
    }
}
