//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use openapi_client::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use openapi_client::{
    Api,
    GetGenerationsResponse,
    GetGenerationsRecordIdResponse,
    GetGroupsResponse,
    GetGroupsRecordIdResponse,
    GetSocialBrandsResponse,
    GetUsersResponse,
    GetUsersRecordIdResponse,
    GetUsersSearchResponse,
    PostLoginResponse,
    PutUsersResponse,
};
use openapi_client::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Get Generations
    async fn get_generations(
        &self,
        r#type: String,
        context: &C) -> Result<GetGenerationsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_generations(\"{}\") - X-Span-ID: {:?}", r#type, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Generation
    async fn get_generations_record_id(
        &self,
        record_id: String,
        context: &C) -> Result<GetGenerationsRecordIdResponse, ApiError>
    {
        let context = context.clone();
        info!("get_generations_record_id(\"{}\") - X-Span-ID: {:?}", record_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Groups
    async fn get_groups(
        &self,
        r#type: String,
        context: &C) -> Result<GetGroupsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_groups(\"{}\") - X-Span-ID: {:?}", r#type, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Group
    async fn get_groups_record_id(
        &self,
        record_id: String,
        context: &C) -> Result<GetGroupsRecordIdResponse, ApiError>
    {
        let context = context.clone();
        info!("get_groups_record_id(\"{}\") - X-Span-ID: {:?}", record_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get SocialBrands
    async fn get_social_brands(
        &self,
        context: &C) -> Result<GetSocialBrandsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_social_brands() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get Users
    async fn get_users(
        &self,
        context: &C) -> Result<GetUsersResponse, ApiError>
    {
        let context = context.clone();
        info!("get_users() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get User
    async fn get_users_record_id(
        &self,
        record_id: String,
        context: &C) -> Result<GetUsersRecordIdResponse, ApiError>
    {
        let context = context.clone();
        info!("get_users_record_id(\"{}\") - X-Span-ID: {:?}", record_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Search Users
    async fn get_users_search(
        &self,
        query: String,
        context: &C) -> Result<GetUsersSearchResponse, ApiError>
    {
        let context = context.clone();
        info!("get_users_search(\"{}\") - X-Span-ID: {:?}", query, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Login
    async fn post_login(
        &self,
        login_request: Option<models::LoginRequest>,
        context: &C) -> Result<PostLoginResponse, ApiError>
    {
        let context = context.clone();
        info!("post_login({:?}) - X-Span-ID: {:?}", login_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update Me
    async fn put_users(
        &self,
        update_user_request: Option<models::UpdateUserRequest>,
        context: &C) -> Result<PutUsersResponse, ApiError>
    {
        let context = context.clone();
        info!("put_users({:?}) - X-Span-ID: {:?}", update_user_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
