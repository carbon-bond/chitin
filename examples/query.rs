use chitin::*;

#[derive(ChitinCodegen)]
pub enum UserDetailQuery {
    #[chitin(request, response = "Result<String, String>")]
    AskUserDetail { user_id: i32 }
}

#[derive(ChitinCodegen)]
pub enum UserQuery {
    #[chitin(request, response = "Result<Vec<String>, String>")]
    AskUserArticles { user_id: i32, count: usize },
    #[chitin(request, response = "Result<Vec<String>, String>")]
    AskUserFriends { user_id: i32 },
    #[chitin(router)]
    UserDetail(UserDetailQuery),
}

#[derive(ChitinCodegen)]
pub enum RootQuery {
    #[chitin(request, response = "Result<Vec<String>, String>")]
    AskArticles { board_id: i32, count: usize },
    #[chitin(request, response = "String")]
    AskBoard { board_id: i32 },
    #[chitin(router)]
    User(UserQuery),
}
