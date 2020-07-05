use crate::api_trait::*;
use crate::Ctx;
use async_trait::async_trait;

#[derive(Default)]
pub struct UserDetail {}

#[async_trait]
impl UserDetailQueryRouter for UserDetail {
    async fn ask_user_detail(
        &self,
        _ctx: Ctx,
        user_id: i32,
    ) -> Result<Option<crate::model::User>, String> {
        let users = crate::USERS.lock().unwrap();
        Ok(users.get(&user_id).map(|u| u.clone()))
    }
}

#[derive(Default)]
pub struct UserQuery {
    user_detail: UserDetail,
}

#[async_trait]
impl UserQueryRouter for UserQuery {
    type UserDetailQueryRouter = UserDetail;
    async fn ask_user_articles(
        &self,
        _ctx: Ctx,
        count: usize,
        user_id: i32,
    ) -> Result<Vec<crate::model::Article>, String> {
        let articles = crate::ARTICLES.lock().unwrap();
        Ok(articles
            .iter()
            .filter(|a| a.author_id == user_id)
            .take(count)
            .cloned()
            .collect())
    }
    async fn login(&self, ctx: Ctx, user_id: i32) -> Result<(), String> {
        ctx.login(user_id);
        Ok(())
    }
    async fn who_am_i(&self, context: Ctx) -> Result<i32, String> {
        context
            .user_id
            .lock()
            .unwrap()
            .ok_or("尚未登入！".to_string())
    }
    fn user_detail_router(&self) -> &Self::UserDetailQueryRouter {
        &self.user_detail
    }
}

#[derive(Default)]
pub struct RootQuery {
    user_query: UserQuery,
}

#[async_trait]
impl RootQueryRouter for RootQuery {
    type UserQueryRouter = UserQuery;
    async fn ask_articles(
        &self,
        _ctx: Ctx,
        count: usize,
    ) -> Result<Vec<crate::model::Article>, String> {
        let articles = crate::ARTICLES.lock().unwrap();
        Ok(articles.iter().take(count).cloned().collect())
    }
    async fn post_article(
        &self,
        _ctx: Ctx,
        article: Option<crate::model::Article>,
    ) -> Result<(), String> {
        let mut articles = crate::ARTICLES.lock().unwrap();
        if let Some(mut article) = article {
            article.created_time = Some(chrono::Utc::now());
            articles.push(article);
        }
        Ok(())
    }
    async fn create_user(&self, _ctx: Ctx, user: crate::model::User) -> Result<i32, String> {
        let mut users = crate::USERS.lock().unwrap();
        let id = users.len() as i32;
        users.insert(id, user);
        Ok(id)
    }
    fn user_router(&self) -> &Self::UserQueryRouter {
        &self.user_query
    }
}
