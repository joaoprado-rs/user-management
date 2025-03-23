use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub reason: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Errors {
    pub errors: Vec<Error>,
}
