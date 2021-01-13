use actix_web::{App, HttpServer};
use database_layer_actix::create_database_pool;
use regex::Regex;

mod util;
mod uuid;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let pool = create_database_pool().await?;

    println!("🚀 Server ready: http://localhost:8080");

    HttpServer::new(move || App::new().data(pool.clone()).configure(uuid::init))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
