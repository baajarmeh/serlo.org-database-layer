use anyhow::Result;
use serde::Serialize;

use super::event::AbstractEvent;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveTaxonomyLink {
    #[serde(flatten)]
    pub abstract_event: AbstractEvent,

    pub parent_id: i32,
    pub child_id: i32,
}

impl RemoveTaxonomyLink {
    pub async fn fetch(abstract_event: AbstractEvent) -> Result<Self> {
        let parent_id = abstract_event.object_id;
        // uses "object" parameter
        let child_id = abstract_event.parameter_uuid_id;

        Ok(RemoveTaxonomyLink {
            abstract_event,

            parent_id,
            child_id,
        })
    }
}
