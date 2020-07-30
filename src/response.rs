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
pub struct ErrorWithMessage {
    pub msg: String,
    pub v: Option<Value>,
}
#[derive(Debug, Clone)]
pub struct Response {
    pub code: StatusCode,
    pub errors: Vec<ErrorWithMessage>,
}
impl Response {
    #[must_use]
    pub fn with(code: StatusCode, msg: String, v: Option<Value>) -> Self {
        Self { code, errors: vec![ErrorWithMessage { msg, v }] }
    }
}

/// @message is to be used to plain english, user facing feedback messages.
/// @error_type may contain additional information from the server, for example 'Database Error'
/// @value: is a json value to be used for application intercommunication purposes.
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize)]
pub struct MessageValue {
    #[serde(default)]
    message: String,
    #[serde(default, rename = "errorType")]
    error_type: Option<String>,
    #[serde(default)]
    value: Option<JsObj>,
}
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "jsonschema", schemars(transparent))]
pub struct JsObj {
    #[serde(flatten)]
    extra: serde_json::Map<String, Value>,
}
impl From<ErrorWithMessage> for MessageValue {
    #[must_use]
    fn from(resp: ErrorWithMessage) -> Self {
        if let Some(v) = resp.v {
            Self::json(resp.msg, v)
        } else {
            Self::new(resp.msg)
        }
    }
}
impl MessageValue {
    #[must_use]
    pub fn build_from(resp: Response) -> Vec<MessageValue> {
        resp.errors.into_iter().map(|err| err.into()).collect()
    }
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message, error_type: None, value: None }
    }

    #[must_use]
    pub fn str(msg: &str) -> Self {
        Self { message: msg.to_owned(), error_type: None, value: None }
    }

    #[must_use]
    pub fn json(message: String, v: Value) -> Self {
        Self {
            message,
            error_type: None,
            value: Some(JsObj { extra: v.as_object().unwrap().to_owned() }),
        }
    }
}
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<MessageValue>,
}
impl From<Response> for ErrorResponse {
    #[must_use]
    fn from(resp: Response) -> Self {
        ErrorResponse { errors: MessageValue::build_from(resp) }
    }
}
impl From<MessageValue> for ErrorResponse {
    #[must_use]
    fn from(mv: MessageValue) -> Self {
        ErrorResponse { errors: vec![mv] }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::MessageValue;
    #[cfg(feature = "jsonschema")]
    use schemars::schema_for;

    #[test]
    fn test_message_value_schema() {
        #[cfg(feature = "jsonschema")]
        {
            let schema = schema_for!(MessageValue);
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
            assert_eq!(
                serde_json::json!({
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "description": "@message is to be used to plain english, user facing feedback messages. @error_type may contain additional information from the server, for example 'Database Error' @value: is a json value to be used for application intercommunication purposes.",
                    "properties": {
                        "errorType": {
                            "default": null,
                            "type": [
                                "string",
                                "null"
                            ]
                        },
                        "message": {
                            "default": "",
                            "type": "string"
                        },
                        "value": {
                            "default": null,
                            "type": [ "object", "null" ],
                            "additionalProperties": true
                        },
                    },
                    "title": "MessageValue",
                    "type": "object"
                }),
                serde_json::json!(&schema)
            );
        }
    }
}
