use uuid::Uuid;

use crate::models::user::{User, UserRequest};

pub fn create_user(req: UserRequest) -> User {
    let uuid = Uuid::new_v4();
    User {
        id: uuid,
        is_active: true,
        username: req.username,
        password: req.password,
        email: req.email,
    }
}
