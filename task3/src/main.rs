use rocket::{launch, post, routes, State};
use task3::SyncService;

#[post("/wait-for-second-party/<unique_id>")]
async fn wait_for_second_party(unique_id: String, sync_service: &State<SyncService>) -> String {
    match sync_service.wait_for_second_party(unique_id).await {
        Ok(msg) => msg.to_string(),
        Err(msg) => msg.to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    let sync_service = SyncService::new();
    rocket::build()
        .manage(sync_service)
        .mount("/", routes![wait_for_second_party])
}
