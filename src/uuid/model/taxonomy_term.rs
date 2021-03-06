use async_trait::async_trait;
use convert_case::{Case, Casing};
use futures::join;
use serde::Serialize;
use sqlx::MySqlPool;

use super::{ConcreteUuid, Uuid, UuidError, UuidFetcher};
use crate::database::Executor;
use crate::format_alias;
use crate::instance::Instance;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxonomyTerm {
    #[serde(rename(serialize = "__typename"))]
    pub __typename: String,
    #[serde(rename(serialize = "type"))]
    pub term_type: String,
    pub instance: Instance,
    pub name: String,
    pub description: Option<String>,
    pub weight: i32,
    pub parent_id: Option<i32>,
    pub children_ids: Vec<i32>,
}

macro_rules! fetch_one_taxonomy_term {
    ($id: expr, $executor: expr) => {
        sqlx::query!(
            r#"
                SELECT u.trashed, term.name, type.name as term_type, instance.subdomain, term_taxonomy.description, term_taxonomy.weight, term_taxonomy.parent_id
                    FROM term_taxonomy
                    JOIN term ON term.id = term_taxonomy.term_id
                    JOIN taxonomy ON taxonomy.id = term_taxonomy.taxonomy_id
                    JOIN type ON type.id = taxonomy.type_id
                    JOIN instance ON instance.id = taxonomy.instance_id
                    JOIN uuid u ON u.id = term_taxonomy.id
                    WHERE term_taxonomy.id = ?
            "#,
            $id
        )
        .fetch_one($executor)
    };
}

macro_rules! fetch_all_entities {
    ($id: expr, $executor: expr) => {
        sqlx::query!(
            r#"
                SELECT entity_id
                    FROM term_taxonomy_entity
                    WHERE term_taxonomy_id = ?
                    ORDER BY position ASC
            "#,
            $id
        )
        .fetch_all($executor)
    };
}

macro_rules! fetch_all_children {
    ($id: expr, $executor: expr) => {
        sqlx::query!(
            r#"
                SELECT id
                    FROM term_taxonomy
                    WHERE parent_id = ?
                    ORDER BY weight ASC
            "#,
            $id
        )
        .fetch_all($executor)
    };
}

macro_rules! fetch_subject {
    ($id: expr, $executor: expr) => {
        TaxonomyTerm::fetch_canonical_subject($id, $executor)
    };
}

macro_rules! to_taxonomy_term {
    ($id: expr, $taxonomy_term: expr, $entities: expr, $children: expr, $subject: expr) => {{
        let taxonomy_term = $taxonomy_term.map_err(|error| match error {
            sqlx::Error::RowNotFound => UuidError::NotFound,
            error => error.into(),
        })?;
        let entities = $entities?;
        let children = $children?;
        let subject = $subject?;

        let mut children_ids: Vec<i32> = entities
            .iter()
            .map(|child| child.entity_id as i32)
            .collect();
        children_ids.extend(children.iter().map(|child| child.id as i32));
        Ok(Uuid {
            id: $id,
            trashed: taxonomy_term.trashed != 0,
            alias: format_alias(subject.as_deref(), $id, Some(&taxonomy_term.name)),
            concrete_uuid: ConcreteUuid::TaxonomyTerm(TaxonomyTerm {
                __typename: "TaxonomyTerm".to_string(),
                term_type: TaxonomyTerm::normalize_type(taxonomy_term.term_type.as_str()),
                instance: taxonomy_term
                    .subdomain
                    .parse()
                    .map_err(|_| UuidError::InvalidInstance)?,
                name: taxonomy_term.name,
                description: taxonomy_term.description,
                weight: taxonomy_term.weight.unwrap_or(0),
                parent_id: taxonomy_term.parent_id.map(|id| id as i32),
                children_ids,
            }),
        })
    }};
}

#[async_trait]
impl UuidFetcher for TaxonomyTerm {
    async fn fetch(id: i32, pool: &MySqlPool) -> Result<Uuid, UuidError> {
        let taxonomy_term = fetch_one_taxonomy_term!(id, pool);
        let entities = fetch_all_entities!(id, pool);
        let children = fetch_all_children!(id, pool);
        let subject = fetch_subject!(id, pool);

        let (taxonomy_term, entities, children, subject) =
            join!(taxonomy_term, entities, children, subject);

        to_taxonomy_term!(id, taxonomy_term, entities, children, subject)
    }

    async fn fetch_via_transaction<'a, E>(id: i32, executor: E) -> Result<Uuid, UuidError>
    where
        E: Executor<'a>,
    {
        let mut transaction = executor.begin().await?;

        let taxonomy_term = fetch_one_taxonomy_term!(id, &mut transaction).await;
        let entities = fetch_all_entities!(id, &mut transaction).await;
        let children = fetch_all_children!(id, &mut transaction).await;
        let subject = fetch_subject!(id, &mut transaction).await;

        transaction.commit().await?;

        to_taxonomy_term!(id, taxonomy_term, entities, children, subject)
    }
}

impl TaxonomyTerm {
    pub async fn fetch_canonical_subject<'a, E>(
        id: i32,
        executor: E,
    ) -> Result<Option<String>, sqlx::Error>
    where
        E: Executor<'a>,
    {
        // Yes, this is super hacky. Didn't find a better way to handle recursion in MySQL 5 (in production, the max depth is around 10 at the moment)
        let subjects = sqlx::query!(
            r#"
                SELECT t.name
                    FROM term_taxonomy t0
                    LEFT JOIN term_taxonomy t1 ON t1.parent_id = t0.id
                    LEFT JOIN term_taxonomy t2 ON t2.parent_id = t1.id
                    LEFT JOIN term_taxonomy t3 ON t3.parent_id = t2.id
                    LEFT JOIN term_taxonomy t4 ON t4.parent_id = t3.id
                    LEFT JOIN term_taxonomy t5 ON t5.parent_id = t4.id
                    LEFT JOIN term_taxonomy t6 ON t6.parent_id = t5.id
                    LEFT JOIN term_taxonomy t7 ON t7.parent_id = t6.id
                    LEFT JOIN term_taxonomy t8 ON t8.parent_id = t7.id
                    LEFT JOIN term_taxonomy t9 ON t9.parent_id = t8.id
                    LEFT JOIN term_taxonomy t10 ON t10.parent_id = t9.id
                    LEFT JOIN term_taxonomy t11 ON t11.parent_id = t10.id
                    LEFT JOIN term_taxonomy t12 ON t12.parent_id = t11.id
                    LEFT JOIN term_taxonomy t13 ON t13.parent_id = t12.id
                    LEFT JOIN term_taxonomy t14 ON t14.parent_id = t13.id
                    LEFT JOIN term_taxonomy t15 ON t15.parent_id = t14.id
                    LEFT JOIN term_taxonomy t16 ON t16.parent_id = t15.id
                    LEFT JOIN term_taxonomy t17 ON t17.parent_id = t16.id
                    LEFT JOIN term_taxonomy t18 ON t18.parent_id = t17.id
                    LEFT JOIN term_taxonomy t19 ON t19.parent_id = t18.id
                    LEFT JOIN term_taxonomy t20 ON t20.parent_id = t19.id
                    JOIN term t on t1.term_id = t.id
                    WHERE
                        t0.parent_id IS NULL AND
                        (
                            t1.id = ? OR t2.id = ? OR t3.id = ? OR t4.id = ? OR t5.id = ? OR t6.id = ? OR t7.id = ? OR t8.id = ? OR t9.id = ? OR t10.id = ? OR
                            t11.id = ? OR t12.id = ? OR t13.id = ? OR t14.id = ? OR t15.id = ? OR t16.id = ? OR t17.id = ? OR t18.id = ? OR t19.id = ? OR t20.id = ?
                        )
            "#,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id,
            id
        ).fetch_one(executor).await;
        match subjects {
            Ok(subject) => Ok(Some(subject.name)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(inner) => Err(inner),
        }
    }

    fn normalize_type(typename: &str) -> String {
        typename.to_case(Case::Camel)
    }
}
