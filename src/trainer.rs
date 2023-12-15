use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Trainer {
    pub name: String,
    pub email: String,
    pub password: String
}