use crate::response::{MessageValue, WebResult};
use actix_web::HttpResponse;

impl From<WebResult> for HttpResponse {
    #[must_use]
    fn from(res: WebResult) -> Self {
        match res {
            WebResult::Ok(v) => Self::Ok().json(&v),
            WebResult::Err(e) => {
                let status_code = e.code;
                let mv: MessageValue = e.into();
                Self::build(status_code).json(&mv)
            }
        }
    }
}
