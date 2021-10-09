use crate::responses::WebResult;
use http::{Response, StatusCode};
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
            WebResult::Err(e) => e
                .errors
                .iter()
                .max_by_key(|e| e.status)
                .map_or_else(
                    || Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR),
                    |large_error| Response::builder().status(large_error.status),
                )
                .body(Body::from(serde_json::to_string(&e).unwrap()))
                .unwrap(),
        }
    }
}
