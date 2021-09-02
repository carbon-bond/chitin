use chitin::*;
use serde::{Deserialize, Serialize};

#[derive(ChitinRouter)]
pub enum UserDetailQuery {
    #[chitin(leaf, response = "Option<crate::model::User>")]
    AskUserDetail { user_id: i32 },
}

#[derive(ChitinRouter)]
pub enum UserQuery {
    #[chitin(leaf, response = "Vec<crate::model::Article>")]
    AskUserArticles { user_id: i32, count: usize },
    #[chitin(leaf, response = "()")]
    Login { user_id: i32 },
    #[chitin(leaf, response = "i32")]
    WhoAmI {},
    #[chitin(router)]
    UserDetail(UserDetailQuery),
}

#[derive(ChitinRouter)]
pub enum RootQuery {
    #[chitin(leaf, response = "Vec<crate::model::Article>")]
    AskArticles { count: usize },
    #[chitin(leaf, response = "()")]
    PostArticle {
        article: Option<crate::model::Article>,
    },
    #[chitin(leaf, response = "i32")]
    CreateUser { user: crate::model::User },
    #[chitin(leaf, response = "crate::model::Test")]
    Test { test: crate::model::Test },
    #[chitin(router)]
    User(UserQuery), // 注意：假如這裡打錯成 `User(i32)` 或其它不是 `ChitinCodegen` 的東西，會報出很難解的錯誤
}
