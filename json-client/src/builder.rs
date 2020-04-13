use crate::RpcClient;

pub struct ClientBuilder {
    url: String,
    user: Option<String>,
    password: Option<String>,
    retry: bool,
}

impl ClientBuilder {
    pub fn new(url: &str) -> ClientBuilder {
        ClientBuilder {
            url: url.to_owned(),
            user: None,
            password: None,
            retry: false,
        }
    }

    pub fn with_retry(mut self) -> Self {
        self.retry = true;
        self
    }

    pub fn build(self) -> RpcClient {
        RpcClient::new(&self.url, self.user, self.password, self.retry)
    }
}
