use std::{sync::Mutex, vec};

use actix_web::{
    get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    let app = web::Data::new(AppState {
        users: Mutex::new(vec![]),
    });

    print!("Starting service at port 8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(app.clone())
            .service(insert_user)
            .service(get_users)
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
