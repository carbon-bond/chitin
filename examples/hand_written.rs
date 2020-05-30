// 這份檔案是「理想上」應該生成的代碼

use async_trait::async_trait;
use serde_json::error::Error;
use crate::query::{RootQuery, UserQuery};

#[async_trait]
pub trait UserRouter {
    async fn ask_user_articles(&self, user_id: i32, board_id: i32) -> Result<String, String>;
    async fn handle_query(&self, query: UserQuery) -> Result<String, Error> {
        match query {
            UserQuery::AskUserArticles { user_id, board_id } => {
                let resp = self.ask_user_articles(user_id, board_id).await;
                serde_json::to_string(&resp)
            }
        }
    }
}
#[async_trait]
pub trait RootQueryRouter {
    type UserRouter: UserRouter + Sync;

    fn user_router(&self) -> &Self::UserRouter;
    async fn ask_articles(&self, board_id: i32, count: u32) -> Result<Vec<String>, String>;
    async fn ask_board(&self, board_id: i32) -> Vec<String>;
    async fn handle_query(&self, query: RootQuery) -> Result<String, Error> {
        match query {
            RootQuery::AskArticles { board_id, count } => {
                let resp = self.ask_articles(board_id, count).await;
                serde_json::to_string(&resp)
            }
            RootQuery::AskBoard { board_id} => {
                let resp = self.ask_board(board_id).await;
                serde_json::to_string(&resp)
            }
            RootQuery::User(query) => {
                self.user_router().handle_query(query).await
            },
            _ => panic!(),
        }
    }
}

