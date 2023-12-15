mod user;

use actix_web::{HttpResponse, HttpServer, App, post, web};
use mongodb::Client;
use crate::user::User;

const DB_NAME: &str = "trainermon";
const COLL_NAME: &str = "users";

#[post("/add_user")]
async fn add_user(client: web::Data<Client>, form: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    
    match result {
        Ok(_) => HttpResponse::Ok().body("User added!"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("Failed to connect db.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(add_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}