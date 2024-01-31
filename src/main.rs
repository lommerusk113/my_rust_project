mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   controllers::run_rest().await
}