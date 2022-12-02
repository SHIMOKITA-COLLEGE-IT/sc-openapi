#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::blacklisted_name)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/mocks/college-app/college-app/406762";
pub const API_VERSION: &str = "1.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetGenerationsResponse {
    /// OK
    OK
    (Vec<models::Generation>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetGenerationsRecordIdResponse {
    /// OK
    OK
    (models::Generation)
    ,
    /// Not Found
    NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetGroupsResponse {
    /// OK
    OK
    (Vec<models::Group>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetGroupsRecordIdResponse {
    /// OK
    OK
    (models::Group)
    ,
    /// Not Found
    NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetSocialBrandsResponse {
    /// OK
    OK
    (Vec<models::SocialBrand>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetUsersResponse {
    /// OK
    OK
    (Vec<models::User>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetUsersRecordIdResponse {
    /// OK
    OK
    (models::User)
    ,
    /// Not Found
    NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetUsersSearchResponse {
    /// OK
    OK
    (Vec<models::User>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum PostLoginResponse {
    /// OK
    OK
    (models::LoginResponse)
    ,
    /// 無効なfirebaseIdToken
    Status401
    ,
    /// Disabledになっている、つまりBANされているアカウント
    Disabled
    ,
    /// Airtable上にデータが存在しません  先にAirtableでEmailだけでも登録してください
    Airtable
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PutUsersResponse {
    /// OK
    OK
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Get Generations
    async fn get_generations(
        &self,
        r#type: String,
        context: &C) -> Result<GetGenerationsResponse, ApiError>;

    /// Get Generation
    async fn get_generations_record_id(
        &self,
        record_id: String,
        context: &C) -> Result<GetGenerationsRecordIdResponse, ApiError>;

    /// Get Groups
    async fn get_groups(
        &self,
        r#type: String,
        context: &C) -> Result<GetGroupsResponse, ApiError>;

    /// Get Group
    async fn get_groups_record_id(
        &self,
        record_id: String,
        context: &C) -> Result<GetGroupsRecordIdResponse, ApiError>;

    /// Get SocialBrands
    async fn get_social_brands(
        &self,
        context: &C) -> Result<GetSocialBrandsResponse, ApiError>;

    /// Get Users
    async fn get_users(
        &self,
        context: &C) -> Result<GetUsersResponse, ApiError>;

    /// Get User
    async fn get_users_record_id(
        &self,
        record_id: String,
        context: &C) -> Result<GetUsersRecordIdResponse, ApiError>;

    /// Search Users
    async fn get_users_search(
        &self,
        query: String,
        context: &C) -> Result<GetUsersSearchResponse, ApiError>;

    /// Login
    async fn post_login(
        &self,
        login_request: Option<models::LoginRequest>,
        context: &C) -> Result<PostLoginResponse, ApiError>;

    /// Update Me
    async fn put_users(
        &self,
        update_user_request: Option<models::UpdateUserRequest>,
        context: &C) -> Result<PutUsersResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Get Generations
    async fn get_generations(
        &self,
        r#type: String,
        ) -> Result<GetGenerationsResponse, ApiError>;

    /// Get Generation
    async fn get_generations_record_id(
        &self,
        record_id: String,
        ) -> Result<GetGenerationsRecordIdResponse, ApiError>;

    /// Get Groups
    async fn get_groups(
        &self,
        r#type: String,
        ) -> Result<GetGroupsResponse, ApiError>;

    /// Get Group
    async fn get_groups_record_id(
        &self,
        record_id: String,
        ) -> Result<GetGroupsRecordIdResponse, ApiError>;

    /// Get SocialBrands
    async fn get_social_brands(
        &self,
        ) -> Result<GetSocialBrandsResponse, ApiError>;

    /// Get Users
    async fn get_users(
        &self,
        ) -> Result<GetUsersResponse, ApiError>;

    /// Get User
    async fn get_users_record_id(
        &self,
        record_id: String,
        ) -> Result<GetUsersRecordIdResponse, ApiError>;

    /// Search Users
    async fn get_users_search(
        &self,
        query: String,
        ) -> Result<GetUsersSearchResponse, ApiError>;

    /// Login
    async fn post_login(
        &self,
        login_request: Option<models::LoginRequest>,
        ) -> Result<PostLoginResponse, ApiError>;

    /// Update Me
    async fn put_users(
        &self,
        update_user_request: Option<models::UpdateUserRequest>,
        ) -> Result<PutUsersResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Get Generations
    async fn get_generations(
        &self,
        r#type: String,
        ) -> Result<GetGenerationsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_generations(r#type, &context).await
    }

    /// Get Generation
    async fn get_generations_record_id(
        &self,
        record_id: String,
        ) -> Result<GetGenerationsRecordIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_generations_record_id(record_id, &context).await
    }

    /// Get Groups
    async fn get_groups(
        &self,
        r#type: String,
        ) -> Result<GetGroupsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_groups(r#type, &context).await
    }

    /// Get Group
    async fn get_groups_record_id(
        &self,
        record_id: String,
        ) -> Result<GetGroupsRecordIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_groups_record_id(record_id, &context).await
    }

    /// Get SocialBrands
    async fn get_social_brands(
        &self,
        ) -> Result<GetSocialBrandsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_social_brands(&context).await
    }

    /// Get Users
    async fn get_users(
        &self,
        ) -> Result<GetUsersResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_users(&context).await
    }

    /// Get User
    async fn get_users_record_id(
        &self,
        record_id: String,
        ) -> Result<GetUsersRecordIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_users_record_id(record_id, &context).await
    }

    /// Search Users
    async fn get_users_search(
        &self,
        query: String,
        ) -> Result<GetUsersSearchResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_users_search(query, &context).await
    }

    /// Login
    async fn post_login(
        &self,
        login_request: Option<models::LoginRequest>,
        ) -> Result<PostLoginResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().post_login(login_request, &context).await
    }

    /// Update Me
    async fn put_users(
        &self,
        update_user_request: Option<models::UpdateUserRequest>,
        ) -> Result<PutUsersResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().put_users(update_user_request, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
