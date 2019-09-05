use futures::lock::Mutex;
use futures::stream::TryStreamExt;
use hyper::client::HttpConnector;
use hyper::Client;
use std::sync::Arc;
use serde_json::json;
use crate::RpcRequest;
use crate::RpcResponse;
use crate::error::Error;
use hyper::Request;
use hyper::Body;

//TODO this should implement some kind of trait that is exposed at the top level I think.
//TODO right now this only uses http, we should make this extendable. TCP, UDP, Http, etc
pub struct RpcClient {
    url: String,
    user: Option<String>,
    password: Option<String>,
    id: Arc<Mutex<u64>>,
    //TODO we should make this runtime library.
    client: Client<HttpConnector>,
}

impl RpcClient {
    pub fn new(url: &str) -> Self {
        RpcClient {
            url: url.to_owned(),
            user: None,
            password: None,
            id: Arc::new(Mutex::new(0)),
            client: Client::new(),
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

        //TODO remove unwrap
        let response = self.send_request(&request).await.unwrap();

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
        // Build request
        // let request_raw = serde_json::to_vec(body)?;
        let request_raw = serde_json::to_vec(body).unwrap(); //TODO

        // let request = Request::builder().method("POST").header("Content-Type", "application/json");
        let mut request_builder = Request::builder();
        request_builder.uri(&self.url).method("POST").header("Content-Type", "application/json");

        if let Some(user) = &self.user {
            //TODO fix this. Need base64 encoding.
            request_builder.header("Authorization", format!("{}{}", user, self.password.clone().unwrap()));
        }

        //TODO remove unwrap
        let request = request_builder.body(Body::from(request_raw)).unwrap();


        //TODO remove unwrap.
        let res = self.client.request(request).await.unwrap();

        //TODO remove unwrap
        let body = res.into_body().try_concat().await.unwrap();

        //TODO remove unwrap
        let rpc_res: R = serde_json::from_slice(&body).unwrap();

        Ok(rpc_res)
    }
}
