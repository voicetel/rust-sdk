//! [`Client`] and [`ClientBuilder`].

use std::sync::{Arc, RwLock};
use std::time::Duration;

use reqwest::Method;

use crate::error::{ApiError, ErrorKind};
use crate::models::account::AccountApiKeyData;
use crate::resources::{
    AccountService, AclService, AuthenticationService, E911Service, GatewaysService,
    INumberingService, LookupsService, MessagingService, NumbersService, SupportService,
};
use crate::transport::Transport;
use crate::version::{DEFAULT_BASE_URL, DEFAULT_USER_AGENT};

/// Configuration builder for [`Client`].
///
/// All fields are optional; the only thing the API server requires of you is a
/// bearer token, and you can supply that either with [`Self::api_key`] or by
/// calling [`Client::login`] after construction.
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    base_url: String,
    api_key: Option<String>,
    user_agent: String,
    timeout: Duration,
    max_retries: u32,
    http: Option<reqwest::Client>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: None,
            user_agent: DEFAULT_USER_AGENT.to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 2,
            http: None,
        }
    }
}

impl ClientBuilder {
    /// Override the API endpoint. Defaults to
    /// [`DEFAULT_BASE_URL`](crate::DEFAULT_BASE_URL). Useful for staging or
    /// `wiremock` in tests.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Install a bearer token. Skip this and call [`Client::login`] instead
    /// if you only have username + password.
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Override the `User-Agent` header sent on every request.
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }

    /// Override the per-request HTTP timeout. Defaults to 30 seconds.
    /// Ignored if [`Self::http_client`] is supplied.
    pub fn timeout(mut self, t: Duration) -> Self {
        self.timeout = t;
        self
    }

    /// Maximum number of automatic retries on 429 / 5xx responses
    /// before bubbling an error. Defaults to 2 (i.e. up to 3 total attempts).
    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    /// Supply a fully-constructed [`reqwest::Client`] (with TLS settings,
    /// pool size, proxy, instrumentation, …) instead of letting the SDK
    /// build one.
    pub fn http_client(mut self, http: reqwest::Client) -> Self {
        self.http = Some(http);
        self
    }

    /// Materialise the [`Client`].
    pub fn build(self) -> Result<Client, ApiError> {
        let http = match self.http {
            Some(c) => c,
            None => reqwest::Client::builder()
                .timeout(self.timeout)
                .build()
                .map_err(|e| ApiError::transport("build HTTP client", e))?,
        };
        let transport = Transport {
            base_url: self.base_url.trim_end_matches('/').to_string(),
            user_agent: self.user_agent,
            max_retries: self.max_retries,
            http,
            api_key: Arc::new(RwLock::new(self.api_key)),
        };
        Ok(Client {
            transport: Arc::new(transport),
        })
    }
}

/// Entry point for the VoiceTel API.
///
/// `Client` is cheap to clone — all internal state is `Arc`-shared. Use the
/// resource accessor methods (`account()`, `numbers()`, …) to perform calls.
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) transport: Arc<Transport>,
}

impl Client {
    /// Begin configuring a new client.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Convenience constructor with all-default settings.
    pub fn new() -> Result<Self, ApiError> {
        Self::builder().build()
    }

    /// The API endpoint this client is configured against.
    pub fn base_url(&self) -> &str {
        &self.transport.base_url
    }

    /// The currently-installed bearer token. Returns `None` before
    /// [`Self::login`] (or [`ClientBuilder::api_key`]) is called.
    pub fn api_key(&self) -> Option<String> {
        self.transport.api_key()
    }

    /// Exchange username + password for a 32-hex API key and install it on
    /// this client. The exchange counts against the 6 req/hour/IP rate limit
    /// shared by every `account/*` endpoint.
    ///
    /// Returns the freshly-issued key for callers who want to persist it.
    pub async fn login(&self, username: u64, password: &str) -> Result<String, ApiError> {
        let body = serde_json::json!({"username": username, "password": password});
        let data: AccountApiKeyData = self
            .transport
            .request(
                Method::POST,
                "/v2.2/account/api-key",
                &[],
                Some(&body),
                false,
            )
            .await?;
        if data.apikey.is_empty() {
            return Err(ApiError::new(
                ErrorKind::Authentication,
                "api-key response did not contain data.apikey",
            ));
        }
        self.transport.set_api_key(Some(data.apikey.clone()));
        Ok(data.apikey)
    }

    // --- resource handles ---------------------------------------------------

    /// `Account` resource handle.
    pub fn account(&self) -> AccountService<'_> {
        AccountService { client: self }
    }
    /// `ACL` resource handle.
    pub fn acl(&self) -> AclService<'_> {
        AclService { client: self }
    }
    /// `Authentication` resource handle.
    pub fn authentication(&self) -> AuthenticationService<'_> {
        AuthenticationService { client: self }
    }
    /// `e911` resource handle.
    pub fn e911(&self) -> E911Service<'_> {
        E911Service { client: self }
    }
    /// `Gateways` resource handle.
    pub fn gateways(&self) -> GatewaysService<'_> {
        GatewaysService { client: self }
    }
    /// `iNumbering` resource handle.
    pub fn i_numbering(&self) -> INumberingService<'_> {
        INumberingService { client: self }
    }
    /// `Lookups` resource handle.
    pub fn lookups(&self) -> LookupsService<'_> {
        LookupsService { client: self }
    }
    /// `Messaging` resource handle.
    pub fn messaging(&self) -> MessagingService<'_> {
        MessagingService { client: self }
    }
    /// `Numbers` resource handle.
    pub fn numbers(&self) -> NumbersService<'_> {
        NumbersService { client: self }
    }
    /// `Support` resource handle.
    pub fn support(&self) -> SupportService<'_> {
        SupportService { client: self }
    }
}
