use crate::responses::{ErrorResponse, WebResult};
use http::Response;
use hyper::Body;

#[allow(clippy::use_self)]
impl From<WebResult> for Response<Body> {
    #[must_use]
    fn from(res: WebResult) -> Self {
        match res {
            WebResult::Ok(v) => {
                let mut resp_builder = Response::builder();
                if let Some(total) = &v.get("full_count") {
                    resp_builder = resp_builder.header("X-Total-Count", total.to_string());
                }
                resp_builder.body(Body::from(serde_json::to_string(&v).unwrap())).unwrap()
            }
            WebResult::Err(e) => {
                let status_code = e.code;
                let mv: ErrorResponse = e.into();
                Response::builder()
                    .status(status_code)
                    .body(Body::from(serde_json::to_string(&mv).unwrap()))
                    .unwrap()
            }
        }
    }
}
