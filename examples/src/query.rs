#![allow(dead_code)]
use chitin::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ChitinCodegen, Debug)]
pub enum UserDetailQuery {
    #[chitin(request, response = "Option<crate::model::User>")]
    AskUserDetail { user_id: i32 },
}

#[derive(Serialize, Deserialize, ChitinCodegen, Debug)]
pub enum UserQuery {
    #[chitin(request, response = "Vec<crate::model::Article>")]
    AskUserArticles { user_id: i32, count: usize },
    #[chitin(request, response = "()")]
    Login { user_id: i32 },
    #[chitin(request, response = "i32")]
    WhoAmI {},
    #[chitin(router)]
    UserDetail(UserDetailQuery),
}

#[derive(Serialize, Deserialize, ChitinCodegen, Debug)]
pub enum RootQuery {
    #[chitin(request, response = "Vec<crate::model::Article>")]
    AskArticles { count: usize },
    #[chitin(request, response = "()")]
    PostArticle {
        article: Option<crate::model::Article>,
    },
    #[chitin(request, response = "i32")]
    CreateUser { user: crate::model::User },
    #[chitin(request, response = "crate::model::Test")]
    Test { test: crate::model::Test },
    #[chitin(router)]
    User(UserQuery), // 注意：假如這裡打錯成 `User(i32)` 或其它不是 `ChitinCodegen` 的東西，會報出很難解的錯誤
}
