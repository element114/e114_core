use crate::responses::WebResult;
use actix_web::HttpResponse;
use http::StatusCode;

impl From<WebResult> for HttpResponse {
    #[must_use]
    fn from(res: WebResult) -> Self {
        match res {
            WebResult::Ok(v) => {
                let mut resp_builder = Self::Ok();
                if let Some(total) = &v.get("full_count") {
                    resp_builder.set_header("X-Total-Count", total.to_string());
                }
                resp_builder.json(&v)
            }
            WebResult::Err(e) => e
                .errors
                .iter()
                .max_by_key(|e| e.status)
                .map_or_else(
                    || Self::build(StatusCode::INTERNAL_SERVER_ERROR),
                    |large_error| Self::build(large_error.status),
                )
                .json(&e),
        }
    }
}
