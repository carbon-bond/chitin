use async_trait::async_trait;
use crate::query2::*;
use serde_json::error::Error;
#[async_trait]
pub trait UserDetailQueryRouter {
    async fn ask_user_detail(&self, context: crate::Ctx, user_id: i32) -> Result<Option<crate::model::User>, String>;
    async fn handle(&self, context: crate::Ctx, query: UserDetailQuery) -> Result<(String, Option<String>), Error> {
        match query {
             UserDetailQuery::AskUserDetail { user_id } => {
                 let resp = self.ask_user_detail(context, user_id).await;
                 let s = serde_json::to_string(&resp)?;
                 Ok((s, resp.err()))
            }
        }
    }
}
#[async_trait]
pub trait UserQueryRouter {
    type UserDetailQueryRouter: UserDetailQueryRouter + Sync;
    async fn ask_user_articles(&self, context: crate::Ctx, user_id: i32, count: usize) -> Result<Vec<crate::model::Article>, String>;
    async fn login(&self, context: crate::Ctx, user_id: i32) -> Result<(), String>;
    async fn who_am_i(&self, context: crate::Ctx, ) -> Result<i32, String>;
   fn user_detail_router(&self) -> &Self::UserDetailQueryRouter;
    async fn handle(&self, context: crate::Ctx, query: UserQuery) -> Result<(String, Option<String>), Error> {
        match query {
             UserQuery::AskUserArticles { user_id, count } => {
                 let resp = self.ask_user_articles(context, user_id, count).await;
                 let s = serde_json::to_string(&resp)?;
                 Ok((s, resp.err()))
            }
             UserQuery::Login { user_id } => {
                 let resp = self.login(context, user_id).await;
                 let s = serde_json::to_string(&resp)?;
                 Ok((s, resp.err()))
            }
             UserQuery::WhoAmI {  } => {
                 let resp = self.who_am_i(context, ).await;
                 let s = serde_json::to_string(&resp)?;
                 Ok((s, resp.err()))
            }
             UserQuery::UserDetail(query) => {
                 self.userDetail_router().handle(context, query).await
            }
        }
    }
}
#[async_trait]
pub trait RootQueryRouter {
    type UserQueryRouter: UserQueryRouter + Sync;
    async fn ask_articles(&self, context: crate::Ctx, count: usize) -> Result<Vec<crate::model::Article>, String>;
    async fn post_article(&self, context: crate::Ctx, article: Option<crate::model::Article>) -> Result<(), String>;
    async fn create_user(&self, context: crate::Ctx, user: crate::model::User) -> Result<i32, String>;
    async fn test(&self, context: crate::Ctx, test: crate::model::Test) -> Result<crate::model::Test, String>;
   fn user_router(&self) -> &Self::UserQueryRouter;
    async fn handle(&self, context: crate::Ctx, query: RootQuery) -> Result<(String, Option<String>), Error> {
        match query {
             RootQuery::AskArticles { count } => {
                 let resp = self.ask_articles(context, count).await;
                 let s = serde_json::to_string(&resp)?;
                 Ok((s, resp.err()))
            }
             RootQuery::PostArticle { article } => {
                 let resp = self.post_article(context, article).await;
                 let s = serde_json::to_string(&resp)?;
                 Ok((s, resp.err()))
            }
             RootQuery::CreateUser { user } => {
                 let resp = self.create_user(context, user).await;
                 let s = serde_json::to_string(&resp)?;
                 Ok((s, resp.err()))
            }
             RootQuery::Test { test } => {
                 let resp = self.test(context, test).await;
                 let s = serde_json::to_string(&resp)?;
                 Ok((s, resp.err()))
            }
             RootQuery::User(query) => {
                 self.user_router().handle(context, query).await
            }
        }
    }
}
