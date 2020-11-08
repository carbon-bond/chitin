use chitin::*;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
pub struct Test {
    pub test: String,
}

#[chitin_model]
mod model {
    use chitin::*;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use typescript_definitions::{TypeScriptify, TypeScriptifyTrait};
    #[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
    pub struct Article {
        pub author_id: i32,
        pub title: String,
        pub content: String,
        pub created_time: Option<DateTime<Utc>>,
    }
    #[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
    pub enum UserType {
        Super,
        Nobody,
    }

    #[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
    pub struct User {
        pub name: String,
        pub sentence: String,
        pub ty: UserType,
    }
    #[chitin_model_use]
    pub use super::Test;
}

pub use model::*;
