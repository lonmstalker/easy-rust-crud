use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use uuid::Uuid;
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub last_name: String,
    pub age: u8,
}

#[derive(Display, Debug, Error)]
#[display(fmt = "catch error: {}", error)]
pub struct CrudError {
    pub(crate) error: &'static str,
}

impl ResponseError for CrudError {}

impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&self)
    }
}
