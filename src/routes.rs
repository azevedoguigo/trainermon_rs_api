use actix_web::{HttpResponse, post, web};
use mongodb::Client;
use crate::{COLL_NAME, DB_NAME};
use crate::trainer::Trainer;

// Trainers routes.
#[post("/add_trainer")]
async fn add_trainer(client: web::Data<Client>, form: web::Form<Trainer>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Trainer added!"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
