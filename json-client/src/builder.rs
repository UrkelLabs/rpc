use crate::RpcClient;

pub struct ClientBuilder {
    url: String,
    user: Option<String>,
    password: Option<String>,
    retry: bool,
    backup_urls: Vec<String>,
}

impl ClientBuilder {
    pub fn new(url: &str) -> ClientBuilder {
        ClientBuilder {
            url: url.to_owned(),
            user: None,
            password: None,
            retry: false,
            backup_urls: Vec::new(),
        }
    }

    pub fn with_retry(mut self) -> Self {
        self.retry = true;
        self
    }

    pub fn with_backups(mut self, backup_urls: Vec<String>) -> Self {
        self.backup_urls = backup_urls;
        self
    }

    pub fn build(self) -> RpcClient {
        RpcClient::new(
            &self.url,
            self.user,
            self.password,
            self.retry,
            self.backup_urls,
        )
    }
}
