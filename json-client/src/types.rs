use crate::error::{Error, RpcError};
use serde_derive::Deserialize;
use serde_derive::Serialize;

//TODO figure out if we can wrap this around the Request struct in a main lib (similar to the error
//type, and then just add on serialization in this specific repo.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RpcRequest {
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub id: serde_json::Value,
    pub jsonrpc: Option<String>,
}

//TODO see above
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RpcResponse {
    pub result: Option<serde_json::Value>,
    pub error: Option<RpcError>,
    pub id: serde_json::Value,
    pub jsonrpc: Option<String>,
}

impl RpcResponse {
    // pub fn result<T: serde::de::DeserializeOwned>(&self) -> Result<T, RpcError> {
    //     if let Some(ref e) = self.error {
    //         return Err(Error::Rpc(e.clone()));
    //     }

    //     serde_json::from_value(self.result.clone().unwrap_or(serde_json::Value::Null))
    //         .map_err(Error::Json)
    // }

    pub fn into_result<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        if let Some(e) = self.error {
            return Err(Error::Rpc(e));
        }

        serde_json::from_value(self.result.unwrap_or(serde_json::Value::Null)).map_err(Error::Json)
    }
}
