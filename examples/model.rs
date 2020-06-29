use chitin::*;

#[chitin_model]
mod model {
    use serde::{Deserialize, Serialize};
    use typescript_definitions::{TypeScriptify, TypeScriptifyTrait};
    #[derive(Serialize, Deserialize, TypeScriptify)]
    pub struct Article {
        pub author: String,
    }

    #[derive(Serialize, Deserialize, TypeScriptify)]
    pub struct User {}
}

pub use model::*;
