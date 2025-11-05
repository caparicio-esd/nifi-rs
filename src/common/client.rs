use std::time::Duration;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Clone)]
pub struct HttpClient{
    client: reqwest::Client,
    auth_token: String,
}

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("HttpClientError::RequestError - {0}")]
    RequestError(reqwest::Error),
    #[error("HttpClientError::HttpError - {status}:{message}")]
    HttpError {
        status: reqwest::StatusCode,
        message: String,
    },
    #[error("HttpClientError::ParseError - {0}")]
    ParseError(reqwest::Error),
}

impl From<reqwest::Error> for HttpClientError {
    fn from(err: reqwest::Error) -> Self {
        HttpClientError::RequestError(err)
    }
}

#[derive(Debug)]
struct JsonResponse<T>(pub T);

#[async_trait]
pub trait ApiResponse: Sized {
    async fn from_response(response: reqwest::Response) -> anyhow::Result<Self, HttpClientError>;
}

#[async_trait]
impl ApiResponse for () {
    async fn from_response(_response: reqwest::Response) -> anyhow::Result<Self, HttpClientError> {
        Ok(())
    }
}

#[async_trait]
impl ApiResponse for String {
    async fn from_response(response: reqwest::Response) -> anyhow::Result<Self, HttpClientError> {
        response.text().await.map_err(HttpClientError::ParseError)
    }
}

#[async_trait]
impl<T> ApiResponse for JsonResponse<T>
where
    T: serde::de::DeserializeOwned + Send,
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
    pub fn new(auth_token: String) -> Self {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(30))
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build reqwest client");
        Self { client, auth_token }
    }

    async fn execute_request(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> anyhow::Result<reqwest::Response, HttpClientError> {
        let response = builder.bearer_auth(&self.auth_token).send().await?;
        let response = response
            .error_for_status()
            .map_err(|err| HttpClientError::HttpError {
                status: err.status().unwrap_or(reqwest::StatusCode::BAD_REQUEST),
                message: err.to_string(),
            })?;
        Ok(response)
    }

    pub async fn get_json<R>(&self, url: &str) -> anyhow::Result<R, HttpClientError>
    where
        R: serde::de::DeserializeOwned,
    {
        let builder = self.client.get(url);
        let response = self.execute_request(builder).await?;
        let json_response = response
            .json::<R>()
            .await
            .map_err(HttpClientError::ParseError)?;
        Ok(json_response)
    }
    pub async fn post<T, R>(&self, url: &str, payload: &T) -> anyhow::Result<R, HttpClientError>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let builder = self.client.post(url).json(payload);
        let response = self.execute_request(builder).await?;
        let json_response = response
            .json::<R>()
            .await
            .map_err(HttpClientError::ParseError)?;
        Ok(json_response)
    }
    pub async fn put<T, R>(&self, url: &str, payload: &T) -> anyhow::Result<R, HttpClientError>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let builder = self.client.put(url).json(payload);
        let response = self.execute_request(builder).await?;
        let json_response = response
            .json::<R>()
            .await
            .map_err(HttpClientError::ParseError)?;
        Ok(json_response)
    }
    pub async fn post_form<T, R>(
        &self,
        url: &str,
        payload: &T,
    ) -> anyhow::Result<R, HttpClientError>
    where
        T: serde::Serialize,
        R: ApiResponse,
    {
        let builder = self.client.post(url).form(payload);
        let response = self.execute_request(builder).await?;
        R::from_response(response).await
    }
}
