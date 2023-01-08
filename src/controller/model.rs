use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use uuid::Uuid;
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PostgresMapper)]
#[serde(rename_all = "camelCase")]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub last_name: String,
    pub age: i32,
}

#[derive(Display, Debug, Error)]
#[display(fmt = "catch error: {}", error)]
pub struct CrudError {
    pub(crate) error: String,
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

impl From<&Row> for User {
    fn from(value: &Row) -> Self {
        User::from_row_ref(value).expect("cant get user from row")
    }
}