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
#[derive(Debug, Clone, Serialize)]
pub struct MessageValue {
    #[serde(default)]
    msg: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    v: Option<Value>,
}

#[cfg(feature = "jsonschema")]
use schemars::schema::{ SchemaObject, InstanceType};
#[cfg(feature = "jsonschema")]
impl JsonSchema for MessageValue {
    fn schema_name() -> std::string::String { 
        "MessageValue".to_owned()
    }
    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { 
        let mut schema = SchemaObject::default();
        schema.instance_type = Some(InstanceType::Object.into());
        let mut str_schema = SchemaObject::default();
        str_schema.instance_type = Some(InstanceType::String.into());
        // str_schema.object().additional_properties = Some(Box::new(true.into()));
        str_schema.metadata().default = Some(serde_json::json!(""));

        let mut v_schema = SchemaObject::default();
        v_schema.instance_type = Some(InstanceType::Object.into());
        v_schema.object().additional_properties = Some(Box::new(true.into()));
        v_schema.metadata().default = Some(serde_json::json!({}));

        let obj = schema.object();
        obj.properties
            .insert("msg".to_owned(), str_schema.into());
        obj.properties
            .insert("v".to_owned(), v_schema.into());
        schema.metadata().description = Some(r#"@msg is to be used to plain english, user facing feedback messages. @v: is a json value to be used for application intercommunication purposes."#.to_owned());
        schema.into()
    }
}

impl From<Response> for MessageValue {
    #[must_use]
    fn from(resp: Response) -> Self {
        if let Some(v) = resp.v {
            Self::json(resp.msg, v)
        } else {
            Self::new(resp.msg)
        }
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
                    "description": "@msg is to be used to plain english, user facing feedback messages. @v: is a json value to be used for application intercommunication purposes.",
                    "properties": {
                        "msg": {
                            "default": "",
                            "type": "string"
                        },
                        "v": {
                            "default": {},
                            "type": "object",
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
