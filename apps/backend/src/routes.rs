use actix_web::{
    HttpResponse, delete, get, post,
    web::{self, Json},
};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::models::{category::Category, flipcard::FlipCard, user::{DeleteingFilter, User}};

#[derive(Deserialize)]
pub struct UserInput {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[post("users/create")]
pub async fn create_user(pool: web::Data<MySqlPool>, json: Json<UserInput>) -> HttpResponse {
    if json.username.is_none() || json.password.is_none() {
        return HttpResponse::BadRequest().finish();
    }
    let email = json.email.as_ref().unwrap();
    let username = json.username.as_ref().unwrap();
    let password = json.password.as_ref().unwrap();

    let user = User::create_user(&pool, email, username, password).await;
    match user {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("users")]
pub async fn get_users(pool: web::Data<MySqlPool>) -> HttpResponse {
    let users = User::get_all_users(&pool).await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[delete("user/{id}")] //TODO: make it more secure by authorization
pub async fn delete_user(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> HttpResponse {
    let id = id.into_inner();
    let result = User::delete_user(&pool, DeleteingFilter::ById(id)).await;

    match result {
        Ok(_) => HttpResponse::Accepted().body("success"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[derive(Deserialize)]
struct FlipCardInput {
    pub first_side: String,
    pub second_side: String,
    pub category_id: i32,
}

#[post("flipcards/create")]
pub async fn create_flipcard(pool: web::Data<MySqlPool>, json: Json<FlipCardInput>) -> HttpResponse {
    let first_side = &json.first_side;
    let second_side = &json.second_side;
    let category_id = json.category_id;
    
    let flipcard = FlipCard::create_flipcard(&pool, first_side, second_side, category_id).await;
    match flipcard {
        Ok(flipcard) => HttpResponse::Created().json(flipcard),
        Err(e) => {
            eprintln!("Error creating flipcard: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize)]
struct CategoryInput {
    pub name: String,
    pub description: String,
    pub creator_id: i32,
}

#[post("categories/create")]
pub async fn create_category(pool: web::Data<MySqlPool>, json: Json<CategoryInput>) -> HttpResponse {
    let name = &json.name;
    let description = &json.description;
    let creator_id = json.creator_id;

    let category = Category::create_category(&pool, name, description, creator_id).await;
    match category {
        Ok(category) => HttpResponse::Created().json(category),
        Err(e) => {
            eprintln!("Error creating category: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("categories")]
pub async fn get_categories(pool: web::Data<MySqlPool>) -> HttpResponse {
    let categories = Category::get_all_categories(&pool).await;
    match categories {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => {
            eprintln!("Error fetching categories: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}