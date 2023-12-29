mod trainer;
mod routes;
pub mod pokemon;

use actix_web::{HttpServer, App, web};
use mongodb::Client;

const DB_NAME: &str = "trainermon";
const TRAINER_COLL_NAME: &str = "trainers";
const POKEMON_COLL_NAME: &str = "pokemons";

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
            .service(routes::add_pokemon)
            .service(routes::get_pokemon)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}