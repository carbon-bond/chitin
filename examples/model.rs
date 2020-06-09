use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub author: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {}
