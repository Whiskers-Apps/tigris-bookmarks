use std::process::exit;

use sniffer_rs::sniffer::Sniffer;
use tigris_rs::features::{
    actions::{
        Field, FieldValidation, OpenFormAction, OpenLinkAction, ResultAction, RunExtensionAction,
        SelectField, SelectFieldValue, SwitchField, TextField,
    },
    api::{send_search_results, GetResultsRequest},
    search::get_search_query,
    search_results::SearchResult,
};

use crate::{bookmarks::get_db, icons::get_icon_path, paths::get_favicon_path};

pub fn handle_results(request: GetResultsRequest) {
    let input_text = request.search_text;
    let search_query = get_search_query(&input_text);
    let keyword = search_query.keyword;
    let search_text = search_query.search_text;
    let mut results = Vec::<SearchResult>::new();
    let sniffer = Sniffer::new();
    let db = get_db();

    if search_text.is_empty() {
        let add_bookmark_result = SearchResult::new("Add Bookmark")
            .set_description("Add a new bookmark")
            .set_icon_color("accent")
            .set_icon_path(&get_icon_path("plus"))
            .set_action(&ResultAction::new_open_form_action(
                &OpenFormAction::new("bookmarks", "add-bookmark", "Add Bookmark", "Add Bookmark")
                    .add_field(&Field::new_text_field(
                        "name",
                        "Name",
                        "The bookmark name",
                        &TextField::new("")
                            .set_validation(&FieldValidation::new().set_not_empty(true)),
                    ))
                    .add_field(&Field::new_text_field(
                        "link",
                        "Link",
                        "The bookmark link",
                        &TextField::new("")
                            .set_validation(&FieldValidation::new().set_not_empty(true)),
                    )),
            ));

        results.push(add_bookmark_result);

        if !db.bookmarks.is_empty() {
            let add_group_result = SearchResult::new("Add Group")
                .set_description("Add a group of bookmarks")
                .set_icon_color("accent")
                .set_icon_path(&get_icon_path("plus"))
                .set_action(&ResultAction::new_open_form_action(
                    &OpenFormAction::new("bookmarks", "add-group", "Add Group", "Add Group")
                        .add_field(&Field::new_text_field(
                            "name",
                            "Name",
                            "The group name",
                            &TextField::new("")
                                .set_validation(&FieldValidation::new().set_not_empty(true)),
                        ))
                        .add_fields(
                            &db.bookmarks
                                .iter()
                                .map(|bookmark| {
                                    Field::new_switch_field(
                                        &bookmark.id.to_string(),
                                        &bookmark.name,
                                        "Select the bookmark if you want it in the group",
                                        &SwitchField::new(false),
                                    )
                                })
                                .collect(),
                        ),
                ));

            let delete_bookmark_result = SearchResult::new("Delete a Bookmark")
                .set_description("Delete a bookmark")
                .set_icon_color("accent")
                .set_icon_path(&get_icon_path("trash"))
                .set_action(&ResultAction::new_open_form_action(
                    &OpenFormAction::new(
                        "bookmarks",
                        "delete-bookmark",
                        "Delete Bookmark",
                        "Delete Bookmark",
                    )
                    .add_field(&Field::new_select_field(
                        "bookmark",
                        "Bookmark",
                        "Select the bookmark you wish to delete",
                        &SelectField::new(
                            &db.bookmarks.first().unwrap().id.to_string(),
                            &db.bookmarks
                                .iter()
                                .map(|bookmark| {
                                    SelectFieldValue::new(&bookmark.id.to_string(), &bookmark.name)
                                })
                                .collect(),
                        ),
                    )),
                ));

            results.push(add_group_result);
            results.push(delete_bookmark_result);
        }

        if !db.groups.is_empty() {
            let delete_group_result = SearchResult::new("Delete Group")
                .set_description("Delete a Group")
                .set_icon_color("accent")
                .set_icon_path(&get_icon_path("trash"))
                .set_action(&ResultAction::new_open_form_action(
                    &OpenFormAction::new(
                        "bookmarks",
                        "delete-group",
                        "Delete Group",
                        "Delete Group",
                    )
                    .add_field(&Field::new_select_field(
                        "group",
                        "Group",
                        "Select the group you wish to delete",
                        &SelectField::new(
                            &db.groups.first().unwrap().id.to_string(),
                            &db.groups
                                .iter()
                                .map(|group| {
                                    SelectFieldValue::new(&group.id.to_string(), &group.name)
                                })
                                .collect(),
                        ),
                    )),
                ));

            results.push(delete_group_result);
        }

        send_search_results(&results);
        exit(0);
    }

    if let Some(keyword) = keyword {
        if &keyword == "e" || &keyword == "edit" {
            let mut edit_bookmark_results = db
                .bookmarks
                .iter()
                .filter(|bookmark| sniffer.matches(&bookmark.name, &search_text))
                .map(|bookmark| {
                    SearchResult::new(&format!("Edit {}", &bookmark.name))
                        .set_description("Edit the bookmark name and url")
                        .set_icon_color("accent")
                        .set_icon_path(&get_icon_path("pencil"))
                        .set_action(&ResultAction::new_open_form_action(
                            &OpenFormAction::new(
                                "bookmarks",
                                "edit-bookmark",
                                "Edit Bookmark",
                                "Save",
                            )
                            .add_arg(&bookmark.id.to_string())
                            .add_field(&Field::new_text_field(
                                "name",
                                "Name",
                                "The bookmark name",
                                &TextField::new(&bookmark.name)
                                    .set_validation(&FieldValidation::new().set_not_empty(true)),
                            ))
                            .add_field(&Field::new_text_field(
                                "link",
                                "Link",
                                "The bookmark link",
                                &TextField::new(&bookmark.link)
                                    .set_validation(&FieldValidation::new().set_not_empty(true)),
                            )),
                        ))
                })
                .collect::<Vec<SearchResult>>();

            let mut edit_group_results = db
                .groups
                .iter()
                .filter(|group| sniffer.matches(&group.name, &search_text))
                .map(|group| {
                    SearchResult::new(&format!("Edit {}", &group.name))
                        .set_description("Edit the group name and bookmarks")
                        .set_icon_color("accent")
                        .set_icon_path(&get_icon_path("pencil"))
                        .set_action(&ResultAction::new_open_form_action(
                            &OpenFormAction::new("bookmarks", "edit-group", "Edit Group", "Save")
                                .add_arg(&group.id.to_string())
                                .add_field(&Field::new_text_field(
                                    "name",
                                    "Name",
                                    "The group name",
                                    &TextField::new(&group.name),
                                ))
                                .add_fields(
                                    &db.bookmarks
                                        .iter()
                                        .map(|bookmark| {
                                            Field::new_switch_field(
                                                &bookmark.id.to_string(),
                                                &bookmark.name,
                                                "Select the bookmark if you want it in the group",
                                                &SwitchField::new(
                                                    group.bookmarks_ids.contains(&bookmark.id),
                                                ),
                                            )
                                        })
                                        .collect(),
                                ),
                        ))
                })
                .collect::<Vec<SearchResult>>();

            results.append(&mut edit_bookmark_results);
            results.append(&mut edit_group_results);

            send_search_results(&results);
            exit(0);
        }
    }

    let mut bookmarks = db
        .bookmarks
        .iter()
        .filter(|bookmark| sniffer.matches(&bookmark.name, &search_text))
        .map(|bookmark| {
            SearchResult::new(&bookmark.name)
                .set_description(&bookmark.link)
                .set_action(&ResultAction::new_open_link_action(&OpenLinkAction::new(
                    &bookmark.link,
                )))
                .set_icon_path(&get_favicon_path(&bookmark.id.to_string()))
        })
        .collect::<Vec<SearchResult>>();

    let mut groups = db
        .groups
        .iter()
        .filter(|group| sniffer.matches(&group.name, &search_text))
        .map(|group| {
            SearchResult::new(&group.name)
                .set_description("Open the group")
                .set_icon_color("accent")
                .set_icon_path(&get_icon_path("folder"))
                .set_action(&ResultAction::new_run_extension_action(
                    &RunExtensionAction::new("bookmarks", "open-group")
                        .add_arg(&group.id.to_string()),
                ))
        })
        .collect::<Vec<SearchResult>>();

    results.append(&mut bookmarks);
    results.append(&mut groups);

    send_search_results(&results);

    exit(0);
}
