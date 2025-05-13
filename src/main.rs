use actions::handle_actions;
use forms::handle_forms;
use results::handle_results;
use tigris_core::features::api::{
    get_request,
    RequestType::{FormResults, GetResults, RunAction},
};

pub mod actions;
pub mod bookmarks;
pub mod forms;
pub mod icons;
pub mod paths;
pub mod results;

#[tokio::main]
async fn main() {
    let request = get_request().unwrap();

    match request.request_type {
        GetResults => {
            handle_results(request.get_results_request.unwrap());
        }
        RunAction => {
            handle_actions(request.run_action_request.unwrap());
        }
        FormResults => {
            tokio::spawn(async {
                handle_forms(request.form_results_request.unwrap()).await;
            })
            .await
            .unwrap();
        }
    }
}
