/// https://nifi.apache.org/docs/nifi-docs/html/administration-guide.html
/// 

pub struct Config {
    pub port_configuration: PortConfiguration,
    pub api_base_url: String,
    pub username: String,
    pub password: String,
    pub(crate) token: Option<String>,
}

pub struct PortConfiguration {
    pub web_https_port: u16,                     // nifi.web.https.port
    pub remote_input_socket_port: Option<u16>,   // nifi.remote.input.socket.port
    pub cluster_node_protocol_port: Option<u16>, // nifi.cluster.node.protocol.port
    pub cluster_node_load_balancing_port: u16,   // nifi.cluster.node.load.balance.port
    pub web_http_forwarding_port: Option<u16>,   // nifi.web.http.port.forwarding
    pub listener_bootstrap_port: u16,            // nifi.listener.bootstrap.port
}

impl Default for PortConfiguration {
    fn default() -> Self {
        Self {
            web_https_port: 8443,
            remote_input_socket_port: Some(10443),
            cluster_node_protocol_port: Some(11443),
            cluster_node_load_balancing_port: 6342,
            web_http_forwarding_port: None,
            listener_bootstrap_port: 0,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port_configuration: Default::default(),
            api_base_url: "https://localhost:8443/nifi-api".to_string(),
            username: "nifi".to_string(),
            password: "nifinifinifinifi".to_string(),
            token: None,
        }
    }
}

impl Config {
    pub fn get_token(&self) -> Option<String> {
        self.token.clone()
    }
    pub fn set_token(&mut self, token: Option<String>) -> Option<String> {
        self.token = token;
        self.token.clone()
    }
}
