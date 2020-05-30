use chitin::*;
use serde::{Deserialize, Serialize};

#[derive(ChitinRouter)]
enum UserQuery {
    #[chitin(request, response = "Result<String, String>")]
    AskUserArticles { user_id: i32, board_id: i32 }, //, count: usize },
}

#[derive(ChitinRouter)]
enum RootQuery {
    #[chitin(request, response = "Result<Vec<String>, String>")]
    AskArticles { board_id: i32, count: u32 }, //, count: usize },
    #[chitin(request, response = "String")]
    AskBoard { board_id: i32 },
    #[chitin(router)]
    User(UserQuery),
}

// use std::future::Future;
// use std::marker::Unpin;
// trait UserRouter {
//     fn ask_user_articles(
//         user_id: i32,
//         board_id: i32,
//     ) -> Box<dyn Future<Output = Result<String, String>>>;
// }
// #[async_trait]
// trait RootQueryRouter {
//     type UserRouter: UserRouter;
//     async fn ask_articles(
//         &mut self,
//         board_id: i32,
//         count: u32,
//     ) -> Result<Vec<String>, String>;
//     fn ask_board(&mut self, board_id: i32) -> Box<dyn Unpin + Future<Output = Vec<String>>>;
//     fn user_router(&mut self) -> &mut Self::UserRouter;
// }

// async fn handle_query<R: RootQueryRouter>(router: &mut R, query: RootQuery) -> String {
//     match query {
//         RootQuery::AskArticles { board_id, count } => {
//             let resp = router.ask_articles(board_id, count).await;
//             let s: Vec<String> = resp.unwrap();
//             serde_json::to_string(&s).unwrap()
//         }
//         // RootQuery::User { query} => router.user_router().
//         _ => panic!(),
//     }
// }

fn main() {
    println!("{}", RootQuery::get_router_name());
    println!("{:?}", RootQuery::get_entries());

    println!("{}", UserQuery::get_router_name());
    println!("{:?}", UserQuery::get_entries());
}
