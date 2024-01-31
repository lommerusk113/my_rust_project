use actix_web::{ HttpResponse, Responder};

use crate::controllers::structs::User;

pub async fn welcome_msg() -> impl Responder {
    HttpResponse::Created().body("Welcome to User RestApi ...")
}

// Create a new user
pub async fn create_user() -> impl Responder {
    // Implement the logic to create a new user
    let user = User {
        id: 1,
        name: "John Doe".to_owned(),
        email: "johndoe@example.com".to_owned(),
    };

    HttpResponse::Created().json(user)
}
