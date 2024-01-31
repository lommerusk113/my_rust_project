use actix_web::{App, HttpServer, web};

pub(crate) mod user_controller;
mod structs;

pub async fn run_rest() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(user_controller::welcome_msg))
            .route("/users", web::post().to(user_controller::create_user))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}