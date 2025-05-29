use std::ptr::null;
use actix_web::{web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, ActiveValue::Set, EntityTrait, ActiveModelTrait};
use log::error;
use crate::entitys::register_entity; // Adjust path if needed
use crate::models::register_model::RegisterModel;     // DTO struct

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub async fn create_user(db: web::Data<DatabaseConnection>, new_user: web::Json<RegisterModel>) -> impl Responder 
{
    println!("Entered registering state");
    //if my front end dev sucks
    // if new_user.name.is_empty() || new_user.username_or_email.is_empty() || new_user.password.is_empty()
    // {
    //     println!("Any one of the params is None");
    //     return HttpResponse::BadRequest().body("Verify that all the params are filled");
    // }

    let org_pass = &new_user.password;
    //let mut rng = OsRng; // get OS-based secure random number generator
    let salt = SaltString::generate(&mut OsRng); //generats random salt form teh os(let salt = SaltString::generate(&mut OsRng);
    //) or even we can hard core our own salt
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(org_pass.as_ref(), &salt) 
    {
        Ok(done) => done.to_string(),
        Err(e) => {
            eprintln!("Password hashing error: {:?}", e);
            return HttpResponse::InternalServerError().body("Password hashing failed");
        }
    };
    


    // Step 1: Create an ActiveModel to represent the data to insert/edit/delete
    let user = register_entity::ActiveModel
    {
        name: Set(new_user.name.clone()),
        username: Set(new_user.username.clone()),
        email:Set(new_user.email.clone()),
        password: Set(password_hash.clone()),//new_user.password.clone()
        ..Default::default()
    };

    // Step 2: Insert into DB using `.insert()` and handle the result
    match user.insert(db.get_ref()).await
    {
        Ok(_) => HttpResponse::Created().body("User created!"),
        Err(err) => {
            eprintln!("Insert error: {:?}", err); // Log the actual error
            HttpResponse::InternalServerError().body(format!("Database error: {}", err))
        }
    }
}
