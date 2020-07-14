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
    HttpError(http_types::Error),
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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref e) => write!(f, "Parse Error: {}", e),
            //@todo
            Error::Rpc(ref e) => write!(f, "RPC Error: {}", e.message),
            Error::Timeout => write!(f, "Timeout error"),
            Error::VersionMismatch => write!(f, "Version Mistmatch"),
            Error::IdMismatch => write!(f, "ID Mismatch"),
            Error::Json(ref e) => write!(f, "JSON Error: {}", e),
            Error::FailedRetry => write!(f, "Failed Retry"),
            Error::HttpError(ref e) => write!(f, "HTTP Error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::ParseError(_) => None,
            //@todo
            Error::Rpc(_) => None,
            Error::Timeout => None,
            Error::VersionMismatch => None,
            Error::IdMismatch => None,
            Error::Json(ref e) => Some(e),
            Error::FailedRetry => None,
            Error::HttpError(_) => None,
        }
    }
}

//JSONRPCERROR should be the error we expose from this specific package.
// JsonRpcError(Error),
