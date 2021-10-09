use http::StatusCode;
use serde::Serialize;
use serde_json::Value;

#[cfg(feature = "jsonschema")]
use schemars::JsonSchema;

/// `WebResult` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
/// This type exists to implement glue code for various web frameworks and.
///
/// [`Ok`]: enum.Result.html#variant.Ok
/// [`Err`]: enum.Result.html#variant.Err
#[derive(Debug)]
#[must_use = "this `Result` may be an `Err` variant, which should be handled"]
pub enum WebResult {
    /// Contains the success value
    Ok(Value),

    /// Contains the error value
    Err(ErrorResponse),
}

impl From<Result<Value, ErrorResponse>> for WebResult {
    fn from(res: Result<Value, ErrorResponse>) -> Self {
        match res {
            Ok(v) => Self::Ok(v),
            Err(e) => Self::Err(e),
        }
    }
}

#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "jsonschema", schemars(transparent))]
pub struct JsObj {
    #[serde(flatten)]
    properties: serde_json::Map<String, Value>,
}

/// # Panics
///
/// Panics if `v.as_object()` fails.
impl From<Value> for JsObj {
    #[must_use]
    fn from(value: Value) -> Self {
        JsObj { properties: value.as_object().unwrap().clone() }
    }
}

/// The standard singular Error object as per <https://jsonapi.org/format/#error-objects>.
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize, Default)]
pub struct ErrorWithMessage {
    /// A human-readable explanation specific to this occurrence of the problem.
    /// Like `title`, this field's value can be localized.
    ///
    /// This used to be `message` pre `0.3`.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub detail: String,

    /// A short, human-readable summary of the problem that SHOULD NOT change from occurrence to occurrence of the problem, except for purposes of localization.
    ///
    /// This used to be `error_type` pre `0.3`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title: Option<String>,

    /// A meta object containing non-standard meta-information about the error.
    /// <https://jsonapi.org/format/#document-meta>
    ///
    /// This used to be `value` pre `0.3`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub meta: Option<JsObj>,

    /// A unique identifier for this particular occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    /// A links object containing at least an `about` member.
    /// Not supported at the moment.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub links: Option<JsObj>,

    /// The HTTP status code applicable to this problem, expressed as a string value.
    ///
    /// This used to be `code` pre `0.3` on the removed `Response` object.
    #[cfg_attr(feature = "jsonschema", schemars(with = "String"))]
    #[serde(
        serialize_with = "http_serde::status_code::serialize",
        deserialize_with = "http_serde::status_code::deserialize",
        default
    )]
    pub status: StatusCode,

    /// An application-specific error code, expressed as a string value.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub code: Option<String>,

    /// An object containing references to the source of the error, optionally including any of the following members:
    ///     - pointer: a JSON Pointer [RFC6901] to the associated entity in the request document [e.g. "/data" for a primary data object, or "/data/attributes/title" for a specific attribute].
    ///     - parameter: a string indicating which URI query parameter caused the error.
    ///
    /// Not supported at the moment.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source: Option<JsObj>,
}

impl ErrorWithMessage {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { detail: message, status: StatusCode::INTERNAL_SERVER_ERROR, ..Self::default() }
    }

    #[must_use]
    pub fn str(msg: &str) -> Self {
        Self {
            detail: msg.to_owned(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            ..Self::default()
        }
    }

    /// # Panics
    ///
    /// Panics if `v.as_object()` fails.
    #[must_use]
    pub fn json(message: String, v: &Value) -> Self {
        Self {
            detail: message,
            meta: Some(JsObj { properties: v.as_object().unwrap().clone() }),
            ..Self::default()
        }
    }
}

/// Error objects MUST be returned as an array keyed by errors in the top level of a `JSON:API`.
/// <https://jsonapi.org/format/#errors>
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<ErrorWithMessage>,
}

impl From<ErrorWithMessage> for ErrorResponse {
    #[must_use]
    fn from(mv: ErrorWithMessage) -> Self {
        ErrorResponse { errors: vec![mv] }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::ErrorWithMessage;
    #[cfg(feature = "jsonschema")]
    use schemars::schema_for;

    #[test]
    fn test_message_value_schema() {
        #[cfg(feature = "jsonschema")]
        {
            let schema = schema_for!(ErrorWithMessage);
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
            assert_eq!(
                serde_json::json!({
                  "$schema": "http://json-schema.org/draft-07/schema#",
                  "title": "ErrorWithMessage",
                  "description": "The standard singular Error object as per <https://jsonapi.org/format/#error-objects>.",
                  "type": "object",
                  "properties": {
                    "code": {
                      "description": "An application-specific error code, expressed as a string value.",
                      "type": [
                        "string",
                        "null"
                      ]
                    },
                    "detail": {
                      "description": "A human-readable explanation specific to this occurrence of the problem. Like `title`, this field's value can be localized.\n\nThis used to be `message` pre `0.3`.",
                      "type": "string"
                    },
                    "id": {
                      "description": "A unique identifier for this particular occurrence of the problem.",
                      "type": [
                        "string",
                        "null"
                      ]
                    },
                    "links": {
                      "description": "A links object containing at least an `about` member. Not supported at the moment.",
                      "type": [
                        "object",
                        "null"
                      ],
                      "additionalProperties": true
                    },
                    "meta": {
                      "description": "A meta object containing non-standard meta-information about the error. <https://jsonapi.org/format/#document-meta>\n\nThis used to be `value` pre `0.3`.",
                      "type": [
                        "object",
                        "null"
                      ],
                      "additionalProperties": true
                    },
                    "source": {
                      "description": "An object containing references to the source of the error, optionally including any of the following members: - pointer: a JSON Pointer [RFC6901] to the associated entity in the request document [e.g. \"/data\" for a primary data object, or \"/data/attributes/title\" for a specific attribute]. - parameter: a string indicating which URI query parameter caused the error.\n\nNot supported at the moment.",
                      "type": [
                        "object",
                        "null"
                      ],
                      "additionalProperties": true
                    },
                    "status": {
                      "description": "The HTTP status code applicable to this problem, expressed as a string value.\n\nThis used to be `code` pre `0.3` on the removed `Response` object.",
                      "default": 200,
                      "type": "string"
                    },
                    "title": {
                      "description": "A short, human-readable summary of the problem that SHOULD NOT change from occurrence to occurrence of the problem, except for purposes of localization.\n\nThis used to be `error_type` pre `0.3`.",
                      "type": [
                        "string",
                        "null"
                      ]
                    }
                  }
                }),
                serde_json::json!(&schema)
            );
        }
    }
}
