use actix_web::{get, web, Responder};
use sqlx::MySqlPool;

use super::messages::AliasQuery;
use crate::instance::Instance;
use crate::message::MessageResponder;

#[get("/alias/{instance}/{path:.*}")]
async fn alias(
    params: web::Path<(Instance, String)>,
    db_pool: web::Data<MySqlPool>,
) -> impl Responder {
    let (instance, path) = params.into_inner();
    let message = AliasQuery { instance, path };
    message.handle(db_pool.get_ref()).await
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(alias);
}
