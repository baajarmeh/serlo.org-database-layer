use std::convert::TryFrom;

use serde::Serialize;

use super::abstract_event::AbstractEvent;
use super::EventError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateComment {
    thread_id: i32,
    comment_id: i32,
}

impl TryFrom<&AbstractEvent> for CreateComment {
    type Error = EventError;

    fn try_from(abstract_event: &AbstractEvent) -> Result<Self, Self::Error> {
        let thread_id = abstract_event.uuid_parameters.try_get("discussion")?;
        let comment_id = abstract_event.object_id;

        Ok(CreateComment {
            thread_id,
            comment_id,
        })
    }
}
