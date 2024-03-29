use chitin::*;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
pub struct Test2 {
    pub s: String,
}

pub mod model_inner {
    use super::*;
    #[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
    pub struct Test {
        pub s: String,
    }
}

#[chitin_model]
mod model_root {
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

    #[chitin_model]
    pub mod embeded_1 {
        use super::*;
        #[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
        pub struct Embeded1Test {
            pub s: String,
        }
        #[chitin_model]
        pub mod embeded_2 {
            use super::*;
            #[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
            pub struct Embeded2Test {
                pub s: String,
            }
        }
    }

    #[derive(Serialize, Deserialize, TypeScriptify, Clone, Debug)]
    pub struct User {
        pub name: String,
        pub sentence: String,
        pub ty: UserType,
    }
    #[chitin_model_use]
    pub use super::{model_inner::Test, Test2};
}

pub use model_root::*;
