//! Types to deserialize.

use json;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RequestId {
    Number(u64),
    String(String),
    Null,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Param {
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestMessage {
    jsonrpc: String,
    id: RequestId,
    method: String,
    params: json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotificationMessage {
    jsonrpc: String,
    method: String,
    params: json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseMessage<D> {
    jsonrpc: String,
    id: RequestId,
    result: Option<json::Value>,
    error: Option<ResponseError<D>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Code {
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseError<D> {
    code: Code,
    message: String,
    data: Option<D>,
}

#[derive(Debug, Clone, Serialize)]
struct Position {
    line: u64,
    character: u64,
}

#[derive(Debug, Clone, Serialize)]
struct Range {
    start: Position,
    end: Position,
}
