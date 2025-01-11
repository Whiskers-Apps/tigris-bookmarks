use std::thread;

use tigris_rs::features::api::RunActionRequest;

use crate::bookmarks::{get_db, Bookmark};

pub fn handle_actions(request: RunActionRequest) {
    let action = request.action;

    match action.as_str() {
        "open-group" => {
            let args = request.args;
            let group_id = args.get(0).unwrap().parse::<usize>().unwrap();
            let db = get_db();

            let group = db.groups.iter().find(|group| group.id == group_id).unwrap();

            let bookmarks = db
                .bookmarks
                .iter()
                .filter(|bookmark| group.bookmarks_ids.contains(&bookmark.id))
                .map(|bookmark| bookmark.to_owned())
                .collect::<Vec<Bookmark>>();

            for bookmark in bookmarks {
                thread::spawn(move || {
                    open::that(&bookmark.link).expect("Error opening bookmark");
                });
            }
        }
        _ => {}
    }
}
