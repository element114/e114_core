use crate::response::{MessageValue, WebResult};
use http::Response;
use hyper::Body;

#[allow(clippy::use_self)]
impl From<WebResult> for Response<Body> {
    #[must_use]
    fn from(res: WebResult) -> Self {
        match res {
            WebResult::Ok(v) => {
                Response::builder().body(Body::from(serde_json::to_string(&v).unwrap())).unwrap()
            }
            WebResult::Err(e) => {
                let status_code = e.code;
                let mv: MessageValue = e.into();
                Response::builder()
                    .status(status_code)
                    .body(Body::from(serde_json::to_string(&mv).unwrap()))
                    .unwrap()
            }
        }
    }
}
