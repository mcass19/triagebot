//! The `events` table provides a way to have scheduled events
use anyhow::{Result, Context as _};
use chrono::{DateTime, FixedOffset};
use tokio_postgres::{Client as DbClient};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub event_id: Uuid,
    pub event_name: String,
    pub expected_event_time: DateTime<FixedOffset>,
    // pub event_metadata: String,
    pub executed_at: DateTime<FixedOffset>,
    pub failed: Option<String>,
}

pub async fn insert_event(db: &DbClient) -> Result<()> {
    unimplemented!();
}

pub async fn delete_event(db: &DbClient) -> Result<()> {
    unimplemented!();
}

pub async fn update_event(db: &DbClient) -> Result<()> {
    unimplemented!();
}

pub async fn get_events_to_execute(db: &DbClient) -> Result<Vec<Event>>  {
    let events = db
        .query(
            "
        SELECT * FROM events",
            &[],
        )
        .await
        .context("Getting events data")?;

    let mut data = Vec::with_capacity(events.len());
    for event in events {
        let event_id: Uuid = event.get(0);
        let event_name: String = event.get(1);
        let expected_event_time: DateTime<FixedOffset> = event.get(2);
        // let event_metadata: String = event.get(3);
        let executed_at: DateTime<FixedOffset> = event.get(4);
        let failed: Option<String> = event.get(5);

        data.push(Event {
            event_id,
            event_name,
            expected_event_time,
            // event_metadata,
            executed_at,
            failed
        });
    }

    Ok(data)
}
