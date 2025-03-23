use std::sync::Mutex;

use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::models::error::{Error, Errors};
use crate::models::user::{User, UserRequest, UserResponse};
use crate::services::user as user_service;

pub struct AppState {
    pub users: Mutex<Vec<User>>,
}

#[post("/users")]
pub async fn insert_user(req: web::Json<UserRequest>, data: web::Data<AppState>) -> impl Responder {
    let user = user_service::create_user(req.into_inner());
    let mut users = data.users.lock().unwrap();
    users.push(user.clone());
    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        is_active: user.is_active,
    };
    let response = serde_json::to_string(&user_response).unwrap();

    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}

#[get("/users")]
pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let vec_users: Vec<UserResponse>;
    let users = data.users.lock().unwrap();

    vec_users = users
        .iter()
        .clone()
        .filter(|user| user.is_active)
        .map(|value| UserResponse {
            id: value.id,
            username: String::from(&value.username),
            is_active: true,
        })
        .collect();

    let request_response = serde_json::to_string(&vec_users).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(request_response)
}

#[get("/users/{id}")]
pub async fn get_user(id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            let mut vec_err: Vec<Error> = Vec::new();
            vec_err.push(Error {
                reason: String::from("Bad Request"),
                message: String::from("Invalid UUID format in the path"),
            });
            return HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&Errors { errors: vec_err }).unwrap());
        }
    };

    let response: HttpResponse = match users.iter().find(|user| user.id == uuid) {
        Some(user) => {
            let user_response = UserResponse {
                id: user.id,
                username: user.username.clone(),
                is_active: user.is_active,
            };
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&user_response).unwrap())
        }
        None => {
            let error_message = Error {
                reason: String::from("user not found"),
                message: format!(
                    "The user {} was not found in the management. Try using `is_active` = false in the query parameter",
                    uuid
                ),
            };
            let mut vec_errors: Vec<Error> = Vec::new();
            vec_errors.push(error_message);
            let errors = Errors { errors: vec_errors };

            HttpResponse::NotFound()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&errors).unwrap())
        }
    };
    response
}
