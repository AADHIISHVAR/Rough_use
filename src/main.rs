use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::Database;
use crate::dto::register_new_user::create_user;

mod post_elli;
mod models;
mod entitys;
mod dto;

#[get("/")]
async fn welcome() -> impl Responder
{
    "welcome to karigalan magic show"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::connect("postgres://postgres:12345678@localhost:5432/postgres").await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/register-user", web::post().to(create_user))
            .service(welcome)
        // your routes here
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
