use crate::error::Error;
use crate::RpcRequest;
use crate::RpcResponse;
use async_std::task::sleep;
use futures::lock::Mutex;
use isahc::Error as IsahcError;
use log::{info, warn};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;

//@todo Auth does not work - fix that.
//TODO this should implement some kind of trait that is exposed at the top level I think.
//TODO right now this only uses http, we should make this extendable. TCP, UDP, Http, etc
//@todo we need a Client builder syntax so that we can enable retry and disable it.
pub struct RpcClient {
    url: String,
    user: Option<String>,
    password: Option<String>,
    id: Arc<Mutex<u64>>,
    retry: bool,
    backup_urls: Vec<String>,
}

impl RpcClient {
    pub fn new(
        url: &str,
        user: Option<String>,
        password: Option<String>,
        retry: bool,
        backup_urls: Vec<String>,
    ) -> Self {
        RpcClient {
            url: url.to_owned(),
            user,
            password,
            id: Arc::new(Mutex::new(0)),
            retry,
            backup_urls,
        }
    }

    async fn build_request(&self, method: &str, params: &[serde_json::value::Value]) -> RpcRequest {
        let mut id = self.id.lock().await;
        *id += 1;
        RpcRequest {
            method: method.to_owned(),
            params: params.to_vec(),
            id: json!(*id),
            jsonrpc: Some("2.0".to_owned()),
        }
    }

    pub async fn execute<T: for<'a> serde::de::Deserialize<'a>>(
        &self,
        method: &str,
        params: &[serde_json::value::Value],
    ) -> Result<T, Error> {
        let request = self.build_request(method, params).await;

        let response = self.send_request(&request).await?;

        Ok(response.into_result()?)
    }

    pub async fn send_request(&self, request: &RpcRequest) -> Result<RpcResponse, Error> {
        let response: RpcResponse = self.send_raw(&request).await?;

        if response.jsonrpc != None && response.jsonrpc != Some(From::from("2.0")) {
            return Err(Error::VersionMismatch);
        }
        if response.id != request.id {
            return Err(Error::IdMismatch);
        }
        Ok(response)
    }

    /// The actual send logic used by both [send_request] and [send_batch].
    async fn send_raw<B, R>(&self, body: &B) -> Result<R, Error>
    where
        B: serde::ser::Serialize,
        R: for<'de> serde::de::Deserialize<'de>,
    {
        let retry_max = 5;
        let mut retries = 0;
        // Build request
        // let request_raw = serde_json::to_vec(body)?;
        // let request_raw = serde_json::to_vec(body).unwrap(); //TODO

        // let request = Request::builder().method("POST").header("Content-Type", "application/json");
        // let mut request_builder = Request::builder();
        let mut current_url = self.url.clone();
        //@todo current only supports 1 backup, let's improve this.
        let current_backup_url = 0;

        loop {
            let mut req = surf::post(&current_url);
            //@todo we might just want to set MIME here actually see: https://docs.rs/surf/1.0.2/surf/struct.Request.html#method.set_mime
            // request_builder.uri(&self.url).method("POST").header("Content-Type", "application/json");

            if let Some(user) = &self.user {
                //TODO fix this. Need base64 encoding.
                req = req.set_header(
                    "Authorization",
                    format!("{}{}", user, self.password.clone().unwrap()),
                );
            }

            let req = req.body_json(body)?;

            match req.recv_json().await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    warn!("RPC Request failed with error: {}", e);

                    //@todo define more conditions in which we'd want to try the backup URL. For
                    //now, we just use timeouts.
                    if let Some(err) = &e.downcast_ref::<IsahcError>() {
                        match err {
                            IsahcError::Timeout => {
                                current_url = self.backup_urls[current_backup_url].clone();
                            }
                            _ => {}
                        }
                    }

                    if !self.retry {
                        return Err(Error::HttpError(e));
                    }
                }
            }

            if self.retry && retries < retry_max {
                retries += 1;
                info!("Retrying request... Retry count: {}", retries);
                //Currently just sleeps the amount of time of retries.
                sleep(Duration::from_secs(retries)).await;
                //Just to be explicit
                continue;
            } else {
                return Err(Error::FailedRetry);
            }
        }
    }
}
