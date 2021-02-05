use actix_web::HttpResponse;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use super::model::{
    Notifications, NotificationsError, SetNotificationStateError, SetNotificationStatePayload,
};
use crate::message::MessageResponder;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum NotificationMessage {
    NotificationsQuery(NotificationsQuery),
    NotificationSetStateMutation(NotificationSetStateMutation),
}

#[async_trait]
impl MessageResponder for NotificationMessage {
    async fn handle(&self, pool: &MySqlPool) -> HttpResponse {
        match self {
            NotificationMessage::NotificationsQuery(message) => message.handle(pool).await,
            NotificationMessage::NotificationSetStateMutation(message) => {
                message.handle(pool).await
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationsQuery {
    pub user_id: i32,
}

#[async_trait]
impl MessageResponder for NotificationsQuery {
    async fn handle(&self, pool: &MySqlPool) -> HttpResponse {
        match Notifications::fetch(self.user_id, pool).await {
            Ok(data) => HttpResponse::Ok()
                .content_type("application/json; charset=utf-8")
                .json(data),
            Err(e) => {
                println!("/notifications/{}: {:?}", self.user_id, e);
                match e {
                    NotificationsError::DatabaseError { .. } => {
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSetStateMutation {
    pub ids: Vec<i32>,
    pub user_id: i32,
    pub unread: bool,
}

#[async_trait]
impl MessageResponder for NotificationSetStateMutation {
    async fn handle(&self, pool: &MySqlPool) -> HttpResponse {
        let payload = SetNotificationStatePayload {
            ids: self.ids.clone(),
            user_id: self.user_id,
            unread: self.unread,
        };
        match Notifications::set_notification_state(payload, pool).await {
            Ok(data) => HttpResponse::Ok()
                .content_type("application/json; charset=utf-8")
                .json(data),
            Err(e) => {
                println!("/set-notification-state: {:?}", e);
                match e {
                    SetNotificationStateError::DatabaseError { .. } => {
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
        }
    }
}
