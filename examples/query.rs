#![allow(dead_code)]

use chitin::*;

#[derive(ChitinCodegen)]
pub enum UserDetailQuery {
    #[chitin(request, response = "Result<crate::model::User, String>")]
    AskUserDetail { user_id: i32 },
}

#[derive(ChitinCodegen)]
pub enum UserQuery {
    #[chitin(request, response = "Result<Vec<String>, String>")]
    AskUserArticles { user_id: i32, count: usize },
    #[chitin(request, response = "Result<Vec<crate::model::User>, String>")]
    AskUserFriends { user_id: i32 },
    #[chitin(router)]
    UserDetail(UserDetailQuery),
}

#[derive(ChitinCodegen)]
pub enum PartyQuery {
    #[chitin(request, response = "Result<Vec<String>, String>")]
    AskPartyMember { id: i32, count: usize },
    #[chitin(request, response = "Result<(), String>")]
    DeleteParty { id: i32 },
}

#[derive(ChitinCodegen)]
pub enum RootQuery {
    #[chitin(request, response = "Result<Vec<String>, String>")]
    AskArticles { board_id: i32, count: usize },
    #[chitin(request, response = "Result<(), String>")]
    PostArticle {
        board_id: i32,
        article: crate::model::Article,
    },
    #[chitin(request, response = "String")]
    AskBoard { board_id: i32 },
    #[chitin(router)]
    User(UserQuery), // 注意：假如這裡打錯成 `User(i32)` 或其它不是 `ChitinCodegen` 的東西，會報出很難解的錯誤
    #[chitin(router)]
    Party(PartyQuery),
}
