#[path = "model.rs"]
mod model;

pub mod crud_config {
    use actix_web::web;
    use crate::controller::crud_controller::{get_route, post_route};

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/user")
                .service(get_route)
                .service(post_route)
        );
    }
}

mod crud_controller {
    use std::collections::HashMap;
    use std::error::Error;
    use actix_web::{get, post, web};
    use deadpool_postgres::Pool;
    use log::info;
    use tokio_postgres::types::ToSql;
    use uuid::Uuid;
    use crate::controller::model::{CrudError, User};


    #[get("/{user_name}")]
    pub async fn get_route(pool: web::Data<Pool>,
                           user_name: web::Path<String>,
                           another: web::Query<HashMap<String, String>>) -> Result<User, Box<dyn Error>> {
        if let Some(data) = another.0.get("another") {
            info!("Some: {}", data)
        }
        let mgr = pool.get().await?;
        let st = mgr
            .prepare_cached("SELECT u.* FROM users u WHERE name = $1")
            .await?;

        let row = mgr.query(&st, &[user_name.as_ref()]).await?.get(0);
        match row {
            None => Err(Box::new(CrudError { error: "user not found" })),
            Some(r) => Ok(User::default())
        }
    }

    #[post("")]
    pub async fn post_route(rq: web::Json<User>) -> Result<User, CrudError> {
        let mut body = rq.0;
        body.id = match body.id {
            Some(_) => return Err(CrudError { error: "unexpected id" }),
            None => Some(Uuid::new_v4())
        };
        Ok(body)
    }
}