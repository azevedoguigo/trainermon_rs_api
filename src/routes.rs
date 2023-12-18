use actix_web::{HttpResponse, post, get, web};
use mongodb::bson::doc;
use mongodb::Client;
use mongodb::Collection;
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

#[get("/get_trainer/{nickname}")]
async fn get_trainer(client: web::Data<Client>, nickname: web::Path<String>) -> HttpResponse {
    let nickname = nickname.into_inner();
    let collection: Collection<Trainer> = client.database(DB_NAME).collection(COLL_NAME);

    match collection.find_one(doc! {"nickname": nickname}, None).await {
        Ok(Some(trainer)) => HttpResponse::Ok().json(trainer),
        Ok(None) => {
            HttpResponse::NotFound().body("Trainer not found!")
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
