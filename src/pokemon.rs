use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Pokemon {
  pub nickname: String,
  pub name: String,
  pub pokemon_type: String,
  pub trainer: String
}