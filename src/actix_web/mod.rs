use crate::response::{MessageValue, WebResult};
use actix_web::HttpResponse;

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
            WebResult::Err(e) => {
                let status_code = e.code;
                let mv: ErrorResponse = e.into();
                Self::build(status_code).json(&mv)
            }
        }
    }
}
