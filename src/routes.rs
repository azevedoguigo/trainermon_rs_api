use actix_web::{HttpResponse, post, get, web};
use mongodb::bson::doc;
use mongodb::Client;
use mongodb::Collection;
use crate::pokemon::Pokemon;
use crate::{TRAINER_COLL_NAME, POKEMON_COLL_NAME, DB_NAME};
use crate::trainer::Trainer;

// Trainers routes.
#[post("/add_trainer")]
async fn add_trainer(client: web::Data<Client>, form: web::Json<Trainer>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(TRAINER_COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Trainer added!"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("/get_trainer/{nickname}")]
async fn get_trainer(client: web::Data<Client>, nickname: web::Path<String>) -> HttpResponse {
    let nickname = nickname.into_inner();
    let collection: Collection<Trainer> = client.database(DB_NAME).collection(TRAINER_COLL_NAME);

    match collection.find_one(doc! {"nickname": nickname}, None).await {
        Ok(Some(trainer)) => HttpResponse::Ok().json(trainer),
        Ok(None) => {
            HttpResponse::NotFound().body("Trainer not found!")
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

// Pokémon routes.
#[post("/add_pokemon")]
async fn add_pokemon(client: web::Data<Client>, form: web::Json<Pokemon>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(POKEMON_COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Pokémon added!"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("/get_pokemon/{nickname}")]
async fn get_pokemon(client: web::Data<Client>, nickname: web::Path<String>) -> HttpResponse {
    let nickname = nickname.into_inner();
    let collection: Collection<Pokemon> = client.database(DB_NAME).collection(POKEMON_COLL_NAME);

    match collection.find_one(doc! {"nickname": nickname}, None).await {
        Ok(Some(pokemon)) => HttpResponse::Ok().json(pokemon),
        Ok(None) => {
            HttpResponse::NotFound().body("Pokémon not found!")
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
