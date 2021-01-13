pub mod util;
pub mod uuid;

use dotenv::dotenv;
use regex::Regex;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::pool::Pool;
use sqlx::MySql;
use std::env;

pub async fn create_database_pool() -> Result<Pool<MySql>, sqlx::Error> {
    dotenv().ok();

    let database_max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .expect("DATABASE_MAX_CONNECTIONS is not set.")
        .parse()
        .unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let re = Regex::new(
        r"^mysql://(?P<username>.+):(?P<password>.+)@(?P<host>.+):(?P<port>\d+)/(?P<database>.+)$",
    )
    .unwrap();
    let captures = re.captures(&database_url).unwrap();
    let username = captures.name("username").unwrap().as_str();
    let password = captures.name("password").unwrap().as_str();
    let host = captures.name("host").unwrap().as_str();
    let port: u16 = captures.name("port").unwrap().as_str().parse().unwrap();
    let database = captures.name("database").unwrap().as_str();

    let options = MySqlConnectOptions::new()
        .host(host)
        .port(port)
        .username(username)
        .password(password)
        .database(database)
        .charset("latin1");
    MySqlPoolOptions::new()
        .max_connections(database_max_connections)
        .connect_with(options)
        .await
}
