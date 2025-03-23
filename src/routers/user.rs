use actix_web::{web, Scope};

use crate::handlers::user::{get_user, get_users, insert_user};

pub fn user_routes() -> Scope {
    web::scope("/users")
        .service(insert_user)
        .service(get_users)
        .service(get_user)
}
