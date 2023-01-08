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
    use deadpool_postgres::Client;
    use log::info;
    use uuid::Uuid;
    use crate::app::app::AppState;
    use crate::controller::model::{CrudError, User};

    #[get("/{user_name}")]
    pub async fn get_route(state: web::Data<AppState>,
                           user_name: web::Path<String>,
                           another: web::Query<HashMap<String, String>>) -> Result<User, Box<dyn Error>> {
        if let Some(data) = another.0.get("another") {
            info!("Some another: {}", data)
        }

        let client: Client = state.pool
            .clone()
            .get()
            .await?;

        let st = client.prepare_cached("SELECT u.* FROM users u WHERE name = $1").await?;

        let rows = client
            .query(&st, &[&*user_name])
            .await?;

        match rows.get(0) {
            None => Err(Box::new(CrudError { error: String::from("user not found") })),
            Some(r) => Ok(r.into())
        }
    }

    #[post("")]
    pub async fn post_route(state: web::Data<AppState>, rq: web::Json<User>) -> Result<User, CrudError> {
        let mut body = rq.0;
        body.id = match body.id {
            Some(_) => return Err(CrudError { error: String::from("unexpected id") }),
            None => Some(Uuid::new_v4())
        };

        let client: Client = state.pool
            .clone()
            .get()
            .await
            .map_err(|e| CrudError { error: e.to_string() })?;

        let st = client.prepare("INSERT INTO users VALUES ($1, $2, $3, $4) RETURNING id, name, last_name, age")
            .await
            .map_err(|e| CrudError { error: e.to_string() })?;

        let result = client.query(&st, &[&body.id.unwrap(), &body.name, &body.last_name, &body.age])
            .await
            .map_err(|e| CrudError { error: e.to_string() })?;

        match result.get(0) {
            None => Err(CrudError { error: String::from("user don't added") }),
            Some(r) => Ok(r.into())
        }
    }
}