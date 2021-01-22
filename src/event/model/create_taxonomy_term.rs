use async_trait::async_trait;
use serde::Serialize;
use sqlx::MySqlPool;

use super::abstract_event::{AbstractEvent, FromAbstractEvent};
use super::EventError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaxonomyTerm {
    #[serde(flatten)]
    abstract_event: AbstractEvent,

    taxonomy_term_id: i32,
}

#[async_trait]
impl FromAbstractEvent for CreateTaxonomyTerm {
    async fn fetch(abstract_event: AbstractEvent, _pool: &MySqlPool) -> Result<Self, EventError> {
        let taxonomy_term_id = abstract_event.object_id;

        Ok(CreateTaxonomyTerm {
            abstract_event,

            taxonomy_term_id,
        })
    }
}