//! # Common HTTP Client Module
//!
//! Provides a robust, cloneable, and thread-safe `HttpClient`
//! that internally manages authentication state.

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::RwLock;

/// A cloneable, async, and state-aware HTTP client for making API requests.
///
/// This client wraps a `reqwest::Client` and is designed to be safely shared
/// across multiple threads and services (e.g., wrapped in an `Arc`).
///
/// It automatically handles connection pooling, timeouts, and the user-agent.
///
/// Its main feature is the management of an **internal, mutable authentication token**.
/// It allows services like an `Access` module to log in (`set_auth_token`), and then
/// all subsequent requests from *any* service sharing this client
/// will automatically include the token.
///
/// It's cheap to clone (`#[derive(Clone)]`) because the internal `reqwest::Client`
/// and the `auth_token` (`Arc<RwLock<...>>`) both use atomic reference counting.
#[derive(Clone, Debug)]
pub struct HttpClient {
    client: reqwest::Client,
    /// The shared, mutable authentication token.
    /// `Arc` makes it shareable, `RwLock` makes it safely mutable.
    /// `Option<String>` represents the state: "logged-in" (`Some(token)`) or "logged-out" (`None`).
    auth_token: Arc<RwLock<Option<String>>>,
}

/// Represents all possible errors that can occur during an HTTP request.
#[derive(Debug, Error)]
pub enum HttpClientError {
    /// A network or request-building error from `reqwest`.
    #[error("HttpClientError::RequestError - {0}")]
    RequestError(reqwest::Error),

    /// An HTTP status error (4xx or 5xx) returned by the server.
    #[error("HttpClientError::HttpError - {status}:{message}")]
    HttpError {
        status: reqwest::StatusCode,
        message: String,
    },

    /// An error during the deserialization (parsing) of the response body.
    #[error("HttpClientError::ParseError - {0}")]
    ParseError(reqwest::Error),
}

/// Allows for automatic conversion from `reqwest::Error` to `HttpClientError` (using `?`).
impl From<reqwest::Error> for HttpClientError {
    fn from(err: reqwest::Error) -> Self {
        HttpClientError::RequestError(err)
    }
}

/// A newtype wrapper to indicate that a response should be deserialized as JSON.
///
/// This is used in generic trait bounds to disambiguate, for example:
/// `client.post_form::<_, JsonResponse<MyStruct>>(...).await`
#[derive(Debug)]
pub struct JsonResponse<T>(pub T);

/// A trait that defines how to parse a `reqwest::Response` into a specific output type.
///
/// This allows our generic methods (`post_form`, `delete`) to return
/// a `String`, a JSON struct (via `JsonResponse`), or nothing (`()`).
#[async_trait]
pub trait ApiResponse: Sized {
    /// Parses the entire response into `Self`.
    ///
    /// # Errors
    /// Returns `HttpClientError::ParseError` if deserialization fails.
    async fn from_response(response: reqwest::Response) -> anyhow::Result<Self, HttpClientError>;
}

/// `ApiResponse` implementation for `()`, for when we don't care about the response body.
#[async_trait]
impl ApiResponse for () {
    async fn from_response(_response: reqwest::Response) -> anyhow::Result<Self, HttpClientError> {
        Ok(()) // Just success, we don't read the body
    }
}

/// `ApiResponse` implementation for `String`, to get the body as plain text.
#[async_trait]
impl ApiResponse for String {
    async fn from_response(response: reqwest::Response) -> anyhow::Result<Self, HttpClientError> {
        response.text().await.map_err(HttpClientError::ParseError)
    }
}

/// `ApiResponse` implementation for `JsonResponse<T>`, for deserializing JSON.
#[async_trait]
impl<T> ApiResponse for JsonResponse<T>
where
    T: DeserializeOwned + Send,
{
    async fn from_response(response: reqwest::Response) -> Result<Self, HttpClientError> {
        let json = response
            .json::<T>()
            .await
            .map_err(HttpClientError::ParseError)?;
        Ok(JsonResponse(json))
    }
}

impl HttpClient {
    /// Creates a new `HttpClient` with default settings.
    ///
    /// The client is initialized without an authentication token (in a "logged-out" state).
    ///
    /// # Panics
    /// Panics if the `reqwest::Client` builder fails (e.g., if the
    /// system's TLS backend cannot be initialized).
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(30))
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
            // Allows self-signed certificates (e.g., from a local NiFi instance)
            // WARNING: Do not use in production unless strictly necessary.
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client,
            // Initialize the token as `None` (logged-out)
            auth_token: Arc::new(RwLock::new(None)),
        }
    }

    /// Safely sets (or overwrites) the internal authentication token.
    ///
    /// Acquires a *write* lock on the token.
    pub async fn set_auth_token(&self, token: String) -> anyhow::Result<()> {
        let mut guard = self.auth_token.write().await;
        *guard = Some(token);
        Ok(())
    }

    /// Safely clears the internal authentication token (for logout).
    ///
    /// Acquires a *write* lock on the token.
    pub async fn clear_auth_token(&self) -> anyhow::Result<()> {
        let mut guard = self.auth_token.write().await;
        *guard = None;
        Ok(())
    }

    /// Gets a clone of the current authentication token, if one exists.
    ///
    /// Acquires a (cheap) *read* lock on the token.
    pub async fn get_auth_token(&self) -> anyhow::Result<Option<String>> {
        let guard = self.auth_token.read().await;
        Ok(guard.clone())
    }

    /// Private helper to execute a request, adding authentication and handling errors.
    ///
    /// 1. Acquires a *read* lock on the token and adds the `Bearer` header if it exists.
    /// 2. Sends the request.
    /// 3. Checks for a successful HTTP status (`error_for_status`), converting 4xx/5xx
    ///    into `HttpClientError::HttpError`.
    async fn execute_request(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> anyhow::Result<reqwest::Response, HttpClientError> {
        // Acquire a read lock
        let token_guard = self.auth_token.read().await;

        // Add the token ONLY if it exists
        let builder = if let Some(token) = token_guard.as_ref() {
            builder.bearer_auth(token)
        } else {
            builder
        };

        let response = builder.send().await?;
        let response = response
            .error_for_status()
            .map_err(|err| HttpClientError::HttpError {
                status: err.status().unwrap_or(reqwest::StatusCode::BAD_REQUEST),
                message: err.to_string(),
            })?;
        Ok(response)
    }

    /// Performs a `GET` request and deserializes the response as JSON.
    ///
    /// `R` is the response type (must be `DeserializeOwned`).
    ///
    /// # Errors
    /// Returns `HttpClientError` on network, HTTP, or parsing failure.
    pub async fn get_json<R>(&self, url: &str) -> anyhow::Result<R, HttpClientError>
    where
        R: DeserializeOwned,
    {
        let builder = self.client.get(url);
        let response = self.execute_request(builder).await?;
        response.json::<R>().await.map_err(HttpClientError::ParseError)
    }

    /// Performs a `POST` request with a JSON payload and deserializes the response as JSON.
    ///
    /// `T` is the payload type (must be `Serialize`).
    /// `R` is the response type (must be `DeserializeOwned`).
    ///
    /// # Errors
    /// Returns `HttpClientError` on network, HTTP, or parsing failure.
    pub async fn post_json<T, R>(&self, url: &str, payload: &T) -> anyhow::Result<R, HttpClientError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let builder = self.client.post(url).json(payload);
        let response = self.execute_request(builder).await?;
        response.json::<R>().await.map_err(HttpClientError::ParseError)
    }

    /// Performs a `PUT` request with a JSON payload and deserializes the response as JSON.
    ///
    /// (Identical to `post_json`, but uses `PUT`).
    ///
    /// # Errors
    /// Returns `HttpClientError` on network, HTTP, or parsing failure.
    pub async fn put_json<T, R>(&self, url: &str, payload: &T) -> anyhow::Result<R, HttpClientError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let builder = self.client.put(url).json(payload);
        let response = self.execute_request(builder).await?;
        response.json::<R>().await.map_err(HttpClientError::ParseError)
    }

    /// Performs a `DELETE` request and parses the response using `ApiResponse`.
    ///
    /// `R` is the response type (must implement `ApiResponse`, e.g., `()`, `String`).
    ///
    /// # Errors
    /// Returns `HttpClientError` on network, HTTP, or parsing failure.
    pub async fn delete<R>(&self, url: &str) -> anyhow::Result<R, HttpClientError>
    where
        R: ApiResponse,
    {
        let builder = self.client.delete(url);
        let response = self.execute_request(builder).await?;
        R::from_response(response).await
    }

    /// Performs a `POST` request with a form payload (`x-www-form-urlencoded`).
    ///
    /// `T` is the payload type (must be `Serialize`).
    /// `R` is the response type (must implement `ApiResponse`, e.g., `()`, `String`).
    ///
    /// # Errors
    /// Returns `HttpClientError` on network, HTTP, or parsing failure.
    ///
    /// # Example
    /// ```no_run
    /// # use serde::{Serialize, Deserialize};
    /// # use nifi_rs::common::client::{HttpClient, JsonResponse, ApiResponse};
    /// #
    /// #[derive(Serialize)]
    /// struct MyForm {
    ///     username: String,
    /// }
    ///
    /// #[derive(Deserialize)]
    /// struct MyResponse {
    ///     token: String,
    /// }
    ///
    /// # async fn run() -> anyhow::Result<()> {
    /// # let client = HttpClient::new();
    /// let form = MyForm { username: "admin".to_string() };
    ///
    /// // Request a response as a String (the token)
    /// let token_str = client.post_form::<_, String>("url", &form).await?;
    ///
    /// // Request a response as JSON
    /// let JsonResponse(resp) = client.post_form::<_, JsonResponse<MyResponse>>("url", &form).await?;
    ///
    /// // Expect no response body, just success
    /// client.post_form::<_, ()>("url", &form).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post_form<T, R>(
        &self,
        url: &str,
        payload: &T,
    ) -> anyhow::Result<R, HttpClientError>
    where
        T: Serialize,
        R: ApiResponse,
    {
        let builder = self.client.post(url).form(payload);
        let response = self.execute_request(builder).await?;
        R::from_response(response).await
    }
}