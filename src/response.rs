use http::StatusCode;
use serde::Serialize;
use serde_json::Value;

#[cfg(feature = "jsonschema")]
use schemars::JsonSchema;

/// `WebResult` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
///
/// [`Ok`]: enum.Result.html#variant.Ok
/// [`Err`]: enum.Result.html#variant.Err
#[derive(Debug)]
#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
pub enum WebResult {
    /// Contains the success value
    Ok(Value),

    /// Contains the error value
    Err(Response),
}

impl From<Result<Value, Response>> for WebResult {
    fn from(res: Result<Value, Response>) -> Self {
        match res {
            Ok(v) => Self::Ok(v),
            Err(e) => Self::Err(e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub code: StatusCode,
    pub msg: String,
    pub v: Option<Value>,
}

impl Response {
    #[must_use]
    pub fn with(code: StatusCode, msg: String, v: Option<Value>) -> Self {
        Self { code, msg, v }
    }
}

/// @msg is to be used to plain english, user facing feedback messages.
/// @v: is a json value to be used for application intercommunication purposes.
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize)]
pub struct MessageValue {
    #[serde(default)]
    msg: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    v: Option<Value>,
}

impl From<Response> for MessageValue {
    #[must_use]
    fn from(resp: Response) -> Self {
        Self { msg: resp.msg, v: resp.v }
    }
}

impl MessageValue {
    #[must_use]
    pub fn new(msg: String) -> Self {
        Self { msg, v: None }
    }

    #[must_use]
    pub fn str(msg: &str) -> Self {
        Self { msg: msg.to_owned(), v: None }
    }

    #[must_use]
    pub fn json(msg: String, v: Value) -> Self {
        Self { msg, v: Some(v) }
    }
}
