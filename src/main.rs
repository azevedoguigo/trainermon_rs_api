mod trainer;
mod routes;

use actix_web::{HttpServer, App, web};
use mongodb::Client;

const DB_NAME: &str = "trainermon";
const COLL_NAME: &str = "trainers";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("Failed to connect db.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(routes::add_trainer)
            .service(routes::get_trainer)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}