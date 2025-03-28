use std::{fs, io::Cursor, process::exit};

use image::{ImageFormat, ImageReader};
use regex::Regex;
use reqwest::Client;
use tigris_rs::features::{api::FormResultsRequest, utils::send_notification};

use crate::{
    bookmarks::{get_db, write_db, Bookmark, Group},
    paths::{get_favicon_path, get_favicons_dir},
};

pub async fn handle_forms(request: FormResultsRequest) {
    let form_id = &request.form_id;

    match form_id.as_str() {
        "add-bookmark" => {
            let name = &request.get_string_value("name").unwrap();
            let link = &request.get_string_value("link").unwrap();

            if !is_valid_link(&link) {
                send_notification("Invalid Link", "The provided link isn't a valid link");
                exit(1);
            }

            let bookmark = Bookmark::new(name, link);
            let mut db = get_db();

            write_favicon(&bookmark.id, &bookmark.link).await;

            db.bookmarks.push(bookmark);

            write_db(&db);

            send_notification("Success", &format!("{name} added successfully"));
            exit(0)
        }
        "delete-bookmark" => {
            let bookmark_id = request.get_usize_value("bookmark").unwrap();
            let mut db = get_db();

            db.bookmarks = db
                .bookmarks
                .iter()
                .filter(|bookmark| bookmark.id != bookmark_id)
                .map(|bookmark| bookmark.to_owned())
                .collect();

            db.groups = db
                .groups
                .iter()
                .map(|group| {
                    let mut group = group.to_owned();

                    if group.bookmarks_ids.contains(&bookmark_id) {
                        group.bookmarks_ids = group
                            .bookmarks_ids
                            .iter()
                            .map(|id| id.to_owned())
                            .filter(|id| id != &bookmark_id)
                            .collect();
                    }

                    group
                })
                .collect();

            write_db(&db);

            let favicon_path = get_favicon_path(&bookmark_id.to_string());

            if favicon_path.exists() {
                fs::remove_file(favicon_path).expect("Error deleting favicon");
            }

            send_notification("Success", "Bookmark deleted successfully");

            exit(0);
        }
        "edit-bookmark" => {
            let args = &request.args;
            let bookmark_id = args.get(0).unwrap().parse::<usize>().unwrap();
            let mut db = get_db();
            let name = &request.get_string_value("name").unwrap();
            let link = &request.get_string_value("link").unwrap();

            if !is_valid_link(&link) {
                send_notification("Invalid Link", "The provided link isn't a valid link");
                exit(1);
            }

            db.bookmarks = db
                .bookmarks
                .iter()
                .map(|bookmark| {
                    if &bookmark.id == &bookmark_id {
                        Bookmark {
                            id: bookmark_id.to_owned(),
                            name: name.to_owned(),
                            link: link.to_owned(),
                        }
                    } else {
                        bookmark.to_owned()
                    }
                })
                .collect();

            write_favicon(&bookmark_id, &link).await;

            write_db(&db);

            send_notification("Success", &format!("{name} edited successfully"));
            exit(0);
        }
        "add-group" => {
            let results = &request.results;
            let name = &request.get_string_value("name").unwrap();
            let selected_bookmarks = results
                .iter()
                .filter(|result| request.get_bool_value(&result.id).unwrap())
                .map(|result| result.id.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let group = Group::new(name, &selected_bookmarks);

            let mut db = get_db();
            db.groups.push(group);

            write_db(&db);

            send_notification("Success", &format!("{} added successfully", name));

            exit(0);
        }
        "edit-group" => {
            let args = &request.args;
            let group_id = args.get(0).unwrap().parse::<usize>().unwrap();
            let name = &request.get_string_value("name").unwrap();

            let mut db = get_db();
            let selected_bookmarks = &request
                .results
                .iter()
                .filter(|result| request.get_bool_value(&result.id).unwrap())
                .map(|result| result.id.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            db.groups = db
                .groups
                .iter()
                .map(|group| {
                    if group.id == group_id {
                        Group {
                            id: group.id,
                            name: name.to_owned(),
                            bookmarks_ids: selected_bookmarks.to_owned(),
                        }
                    } else {
                        group.to_owned()
                    }
                })
                .collect();

            write_db(&db);

            send_notification("Success", &format!("{name} edited successfully"));

            exit(0)
        }
        "delete-group" => {
            let group_id = request.get_usize_value("group").unwrap();
            let mut db = get_db();

            db.groups = db
                .groups
                .iter()
                .filter(|group| group.id != group_id)
                .map(|group| group.to_owned())
                .collect();

            write_db(&db);

            send_notification("Success", "Group deleted successfully");

            exit(0);
        }
        _ => {}
    }
}

async fn write_favicon(id: &usize, link: &str) {
    let website = link.replace("https://", "").replace("http://", "");

    let favicon_request = Client::new()
        .get(format!(
            "https://favicon.is/{website}?larger=true"
        ))
        .send()
        .await;

    if let Ok(response) = favicon_request {
        if response.status().is_success() {
            let bytes = response.bytes().await.unwrap();

            if !get_favicons_dir().exists() {
                fs::create_dir_all(get_favicons_dir()).expect("Error creating favicons directory");
            }

            let favicon_path = get_favicon_path(&id.to_string());

            let image = ImageReader::new(Cursor::new(&bytes))
                .with_guessed_format()
                .unwrap()
                .decode()
                .expect("Error converting bytes to image");

            image
                .save_with_format(&favicon_path, ImageFormat::Png)
                .expect("Error saving image");
        }
    }
}

fn is_valid_link(link: &str) -> bool {
    let url_regex = Regex::new(
        r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)",
    ).unwrap();

    url_regex.is_match(&link)
}
