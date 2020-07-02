#![allow(dead_code)]
use chitin::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ChitinCodegen)]
pub enum UserDetailQuery {
    #[chitin(request, response = "(crate::model::User, String)")]
    AskUserDetail { user_id: i32 },
}

#[derive(Serialize, Deserialize, ChitinCodegen)]
pub enum UserQuery {
    #[chitin(request, response = "Vec<String>")]
    AskUserArticles { user_id: i32, count: usize },
    #[chitin(request, response = "Vec<crate::model::User>")]
    AskUserFriends { user_id: i32 },
    #[chitin(router)]
    UserDetail(UserDetailQuery),
}

#[derive(Serialize, Deserialize, ChitinCodegen)]
pub enum PartyQuery {
    #[chitin(request, response = "Vec<String>")]
    AskPartyMember { id: i32, count: usize },
    #[chitin(request, response = "()")]
    DeleteParty { id: i32 },
}

#[derive(Serialize, Deserialize, ChitinCodegen)]
pub enum RootQuery {
    #[chitin(request, response = "Vec<String>")]
    AskArticles { board_id: i32, count: usize },
    #[chitin(request, response = "()")]
    PostArticle {
        board_id: i32,
        article: crate::model::Article,
    },
    #[chitin(request, response = "String")]
    AskBoard { board_id: Option<i32> },
    #[chitin(router)]
    User(UserQuery), // 注意：假如這裡打錯成 `User(i32)` 或其它不是 `ChitinCodegen` 的東西，會報出很難解的錯誤
    #[chitin(router)]
    Party(PartyQuery),
}
