use actix_web::{web, HttpResponse, Responder};
use jwt_simple::prelude::*;
use sea_orm::DatabaseConnection;
use crate::models::login_model::LoginModel;

use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use crate::entitys::register_entity::Entity as Register;
use crate::entitys::register_entity::Column;
use crate::models;

pub fn create_jwt_key() -> Result<String, Box<dyn std::error::Error>> {
    let key = HS256Key::generate();
    let claims = Claims::create(Duration::from_hours(2)); // 2 hours
    let token = key.authenticate(claims)?;
    verify_tkn(&key, &token)?;
    Ok(token)
}   

pub fn verify_tkn(key: &HS256Key, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _claims = key.verify_token::<NoCustomClaims>(token, None)?;
    Ok(())
}

//todo do teh auth part later

async fn get_user_by_username(
    db: web::Data<sea_orm::DatabaseConnection>,
    new_user: web::Json<LoginModel>,
) -> impl Responder {
    let name = new_user.into_inner();
    println!("{:?}",name);

    match Register::find()
        .filter(Column::Username.eq(name.username_or_email.clone()))
        .one(db.get_ref())
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(name),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            eprintln!("Database error: {}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
