use std::{net::SocketAddr, time::Duration};
use std::env::VarError;
use oauth2::basic::BasicClient;
// use oauth2::reqwest;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

use std::env;
use std::error::Error;

use reqwest::ClientBuilder;

use crate::{
    config::{Config},
    Client, OxideError,
};

pub struct Context {
    client: Option<Client>,
    config: Config,
}

impl Context {
    pub async fn new(config: Config) -> Result<Self, OxideError> {
        let client = get_client(&config).await.unwrap();
        Ok(Self { client, config })
    }

    pub fn client(&self) -> Result<&Client, OxideError> {
        self.client
            .as_ref()
            .ok_or_else(|| OxideError::NoAuthenticatedHosts)
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

async fn get_client(config: &Config) -> Result<Option<Client>, OxideError> {
    Ok(Some(make_client(&config).await))
}

pub async fn make_client(config: &Config) -> Client {
    let config = config.clone();
    let endpoint = &config.endpoint;

    Client::new_with_client(endpoint.as_str(), make_rclient(config.clone()).await.unwrap().build().unwrap())
}

pub async fn make_rclient(config: &Config) -> Result<reqwest::ClientBuilder, Box<dyn Error>> {
    let mut client_builder = ClientBuilder::new().connect_timeout(Duration::from_secs(15));

    let token_url = TokenUrl::new("https://www.ovh.com/auth/oauth2/token".to_string())
        .expect("Invalid token endpoint URL");

    let client = BasicClient::new(
        ClientId::new(config.client_id.clone()),
        Some(ClientSecret::new(config.client_secret.clone())),
        AuthUrl::new("http://authorize".to_string()).unwrap(),
        Some(token_url)
    );

    let response = client
        .exchange_client_credentials()
        .add_scope(Scope::new("all".to_string()))
        .request_async(async_http_client).await.unwrap();

    let token = response.access_token().secret();

    // let mut client = &self.client;



    let mut bearer =
        reqwest::header::HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap();
    bearer.set_sensitive(true);
    client_builder = client_builder.default_headers(
        [(reqwest::header::AUTHORIZATION, bearer)]
            .into_iter()
            .collect(),
    );

    /* let c = config.clone();



    if let Some(token) = token {
        let mut bearer =
            reqwest::header::HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap();
        bearer.set_sensitive(true);
        client_builder = client_builder.default_headers(
            [(reqwest::header::AUTHORIZATION, bearer)]
                .into_iter()
                .collect(),
        );
    }*/

    Ok(client_builder)
}