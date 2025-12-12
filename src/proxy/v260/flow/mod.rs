use crate::common::client::HttpClient;
use crate::common::config::Config;
use crate::proxy::v260::api::RegisteredFlowSnapshot;
use std::sync::Arc;

#[derive(Debug)]
pub struct Flow {
    /// A thread-safe, shared HTTP client for making API requests.
    client: Arc<HttpClient>,
    /// A thread-safe, shared configuration object, primarily for the API base URL.
    config: Arc<Config>,
}

impl Flow {
    pub fn new(client: Arc<HttpClient>, config: Arc<Config>) -> Self {
        Self { client, config }
    }

    pub async fn get_root_flow(&self) -> anyhow::Result<RegisteredFlowSnapshot> {
        let response = self
            .client
            .get_json::<RegisteredFlowSnapshot>(&format!(
                "{}/process-groups/root/download",
                self.config.api_base_url
            ))
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use crate::common::client::HttpClient;
    use crate::common::config::Config;
    use crate::proxy::v260::access::Access;
    use crate::proxy::v260::flow::Flow;
    use std::sync::Arc;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_get_root_flow() {
        let client = Arc::new(HttpClient::new());
        let config = Arc::new(Config::default()); // Assumes correct credentials
        let access = Access::new(client.clone(), config.clone());
        let _ = access.get_access_token().await;
        let root_flow = Flow::new(client.clone(), config.clone());
        let flow = root_flow.get_root_flow().await;
        assert!(flow.is_ok(), "test_get_root_flow call error: {:?}", flow);
        let flow = flow.unwrap();
        dbg!(&flow);
    }
}
