use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    Rpc(RpcError),
    Timeout,
    //TODO rename
    VersionMismatch,
    //TODO rename
    IdMismatch,
    Json(serde_json::Error),
    FailedRetry,
    HttpError(std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>),
    // FailedRetries(
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

//TODO pull this out into the main package into rpc-types something like that.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RpcError {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

//TODO make this work so that servers can add more errors onto it.
#[derive(Debug)]
pub enum RpcErrorTypes {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
}

impl RpcError {
    pub fn from_error_type(error: RpcErrorTypes, data: Option<serde_json::Value>) -> Self {
        match error {
            RpcErrorTypes::ParseError => Self {
                code: -32700,
                message: "Parse Error".to_owned(),
                data,
            },
            RpcErrorTypes::InvalidRequest => Self {
                code: -32600,
                message: "Invalid Request".to_owned(),
                data,
            },
            RpcErrorTypes::MethodNotFound => Self {
                code: -32601,
                message: "Method Not Found".to_owned(),
                data,
            },
            RpcErrorTypes::InvalidParams => Self {
                code: -32602,
                message: "Invalid Params".to_owned(),
                data,
            },
            RpcErrorTypes::InternalError => Self {
                code: -32603,
                message: "Internal Error".to_owned(),
                data,
            },
        }
    }
}

//JSONRPCERROR should be the error we expose from this specific package.
// JsonRpcError(Error),
