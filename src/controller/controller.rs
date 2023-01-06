pub mod crud_config {
    use actix_web::web;
    use crate::controller::crud_controller::{get_route, post_route};

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/crud")
                .service(get_route)
                .service(post_route)
        );
    }
}

mod crud_controller {
    use std::collections::HashMap;
    use actix_web::{get, HttpRequest, HttpResponse, post, Responder, ResponseError, web};
    use actix_web::body::BoxBody;
    use actix_web::http::header::ContentType;
    use derive_more::{Display, Error};
    use log::info;
    use uuid::Uuid;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    struct MyReq {
        id: Option<Uuid>,
        name: String,
        age: u8,
    }

    #[derive(Display, Debug, Error)]
    #[display(fmt = "catch error: {}", error)]
    struct CrudError {
        error: &'static str,
    }

    impl ResponseError for CrudError {}

    impl Responder for MyReq {
        type Body = BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(&self)
        }
    }

    #[get("/{user_name}")]
    pub async fn get_route(user_name: web::Path<String>, another: web::Query<HashMap<String, String>>) -> String {
        if let Some(data) = another.0.get("another") {
            info!("Some: {}", data)
        }
        format!("Hello, {user_name}!")
    }

    #[post("")]
    pub async fn post_route(rq: web::Json<MyReq>) -> Result<MyReq, CrudError> {
        let mut body = rq.0;
        body.id = match body.id {
            Some(_) => return Err(CrudError { error: "unexpected id" }),
            None => Some(Uuid::new_v4())
        };
        Ok(body)
    }
}