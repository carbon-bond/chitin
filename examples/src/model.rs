use chitin::*;

#[chitin_model]
mod model {
    use serde::{Deserialize, Serialize};
    use typescript_definitions::{TypeScriptify, TypeScriptifyTrait};
    #[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
    pub struct Article {
        pub author_id: i32,
        pub title: String,
        pub content: String,
    }

    #[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
    pub struct User {
        pub name: String,
        pub sentence: String,
    }
}

pub use model::*;
