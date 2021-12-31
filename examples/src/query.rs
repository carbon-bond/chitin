use chitin::*;
use serde::{Deserialize, Serialize};

#[derive(ChitinRouter, Deserialize, Serialize, Debug)]
pub enum UserDetailQuery {
    #[chitin(leaf, response = "Option<crate::model_root::User>")]
    AskUserDetail { user_id: i32 },
}

#[derive(ChitinRouter, Deserialize, Serialize, Debug)]
pub enum UserQuery {
    #[chitin(leaf, response = "Vec<crate::model_root::Article>")]
    AskUserArticles { user_id: i32, count: usize },
    #[chitin(leaf, response = "()")]
    Login { user_id: i32 },
    #[chitin(leaf, response = "i32")]
    WhoAmI {},
    #[chitin(router)]
    UserDetail(UserDetailQuery),
}

#[derive(ChitinRouter, Deserialize, Serialize, Debug)]
pub enum RootQuery {
    #[chitin(leaf, response = "Vec<crate::model_root::Article>")]
    AskArticles { count: usize },
    #[chitin(leaf, response = "()")]
    PostArticle {
        article: Option<crate::model_root::Article>,
    },
    #[chitin(leaf, response = "i32")]
    CreateUser { user: crate::model_root::User },
    #[chitin(leaf, response = "crate::model_root::Test")]
    Test { test: crate::model_root::Test },
    #[chitin(leaf, response = "crate::model_root::embeded_1::Embeded1Test")]
    Embeded1Test {
        test: crate::model_root::embeded_1::Embeded1Test,
    },
    #[chitin(
        leaf,
        response = "crate::model_root::embeded_1::embeded_2::Embeded2Test"
    )]
    Embeded2Test {
        test: crate::model_root::embeded_1::embeded_2::Embeded2Test,
    },
    #[chitin(router)]
    User(UserQuery), // 注意：假如這裡打錯成 `User(i32)` 或其它不是 `ChitinCodegen` 的東西，會報出很難解的錯誤
}
