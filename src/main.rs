use std::{sync::Mutex, vec};

use actix_web::{
    get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Error {
    reason: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct Errors {
    errors: Vec<Error>,
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: Uuid,
    username: String,
    password: String,
    email: String,
    is_active: bool,
}

#[derive(Serialize, Deserialize)]
struct UserRequest {
    username: String,
    password: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct UserResponse {
    id: Uuid,
    username: String,
    is_active: bool,
}

struct AppState {
    users: Mutex<Vec<User>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print!("Starting service at port 8080...");
    let app = web::Data::new(AppState {
        users: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app.clone())
            .service(insert_user)
            .service(get_users)
            .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/users")]
async fn insert_user(req: web::Json<UserRequest>, data: web::Data<AppState>) -> impl Responder {
    let user = extract_and_convert_body(req);
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

fn extract_and_convert_body(req: web::Json<UserRequest>) -> User {
    let uuid = Uuid::new_v4();
    let UserRequest {
        username,
        password,
        email,
    } = req.into_inner();

    User {
        id: uuid,
        is_active: true,
        username,
        password,
        email,
    }
}

#[get("/users")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
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
async fn get_user(id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            let mut vec_err: Vec<Error> = Vec::new();
            vec_err.push(Error {
                reason: String::from("Bad Request"),
                message: String::from("Invalid UUID format in the path"),
            });
            return HttpResponse::NotFound()
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
            HttpResponse::NotFound()
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
