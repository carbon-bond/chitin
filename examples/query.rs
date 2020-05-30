use chitin::*;

#[derive(ChitinCodegen)]
pub enum UserQuery {
    #[chitin(request, response = "Result<String, String>")]
    AskUserArticles { user_id: i32, board_id: i32 }, //, count: usize },
}

#[derive(ChitinCodegen)]
pub enum RootQuery {
    #[chitin(request, response = "Result<Vec<String>, String>")]
    AskArticles { board_id: i32, count: u32 }, //, count: usize },
    #[chitin(request, response = "String")]
    AskBoard { board_id: i32 },
    #[chitin(router)]
    User(UserQuery),
}
