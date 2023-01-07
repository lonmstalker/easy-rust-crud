use std::io::ErrorKind;
use log::{debug, error, info};
use tokio_postgres::NoTls;
use crate::datasource::datasource::config::{Error, PostgresProperties};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("resources/migrations");
}

pub async fn run_migrations(props: &PostgresProperties) -> Result<(), std::io::Error> {
    info!("migrations started");

    let cfg = "host=".to_owned() + &*props.db_host.clone()
        + " user=" + &*props.db_user.clone()
        + " password=" + &*props.db_password.clone()
        + " port=" + &props.db_port.clone().to_string()
        + " dbname=" + &*props.db_name.clone();

    let (mut client, con) = tokio_postgres::connect(&*cfg, NoTls).await
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))?;

    tokio::spawn(async move {
        if let Err(e) = con.await {
            error!("connection error: {}", e);
        }
    });

    let migration_report = embedded::migrations::runner()
        .run_async(&mut client)
        .await
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))?;

    for migration in migration_report.applied_migrations() {
        debug!("Migration applied: Version = {}, name = {}", migration.version(), migration.name());
    }

    info!("migrations finished");

    Ok(())
}