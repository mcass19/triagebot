//! The `jobs` table provides a way to have scheduled jobs
use anyhow::{Result, Context as _};
use chrono::{DateTime, FixedOffset, Duration};
use tokio_postgres::{Client as DbClient};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use postgres_types::{ToSql, FromSql};

const DAY_IN_SECONDS: i32 = 86400;
const HOUR_IN_SECONDS: i32 = 3600;
const MINUTE_IN_SECONDS: i32 = 60;

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub job_type: JobType,
    pub expected_time: DateTime<FixedOffset>,
    pub cron_period: Option<i32>,
    pub cron_unit: Option<CronUnit>,
    pub metadata: serde_json::Value,
    pub executed_at: Option<DateTime<FixedOffset>>,
    pub error_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSql, FromSql)]
#[postgres(name = "job_type")]
pub enum JobType {
    #[postgres(name = "cron")]
    Cron,
    #[postgres(name = "single_execution")]
    SingleExecution
}

#[derive(Serialize, Deserialize, Debug, ToSql, FromSql)]
#[postgres(name = "cron_unit")]
pub enum CronUnit {
    #[postgres(name = "day")]
    Day,
    #[postgres(name = "hour")]
    Hour,
    #[postgres(name = "minute")]
    Minute,
    #[postgres(name = "second")]
    Second,
}

pub async fn insert_job(
    db: &DbClient, 
    name: &String,
    job_type: &JobType,
    expected_time: &DateTime<FixedOffset>,
    cron_period: &Option<i32>,
    cron_unit: &Option<CronUnit>,
    metadata: &serde_json::Value
) -> Result<()> {
    tracing::trace!("insert_job(name={})", name);
    
    db.execute(
        "INSERT INTO jobs (name, job_type, expected_time, cron_period, cron_unit, metadata) VALUES ($1, $2, $3, $4, $5, $6) 
            ON CONFLICT (name, expected_time) DO UPDATE SET metadata = EXCLUDED.metadata",
        &[&name, &job_type, &expected_time, &cron_period, &cron_unit, &metadata],
    )
    .await
    .context("Inserting job")?;

    Ok(())
}

pub async fn delete_job(db: &DbClient, id: &Uuid) -> Result<()> {
    tracing::trace!("delete_job(id={})", id);
    
    db.execute(
        "DELETE FROM jobs WHERE id = $1",
        &[&id],
    )
    .await
    .context("Deleting job")?;

    Ok(())
}

pub async fn update_job_error_message(db: &DbClient, id: &Uuid, message: &String) -> Result<()> {
    tracing::trace!("update_job_error_message(id={})", id);
    
    db.execute(
        "UPDATE jobs SET error_message = $2 WHERE id = $1",
        &[&id, &message],
    )
    .await
    .context("Updating job error message")?;

    Ok(())
}

pub async fn update_job_executed_at(db: &DbClient, id: &Uuid) -> Result<()> {
    tracing::trace!("update_job_executed_at(id={})", id);
    
    db.execute(
        "UPDATE jobs SET executed_at = now() WHERE id = $1",
        &[&id],
    )
    .await
    .context("Updating job executed at")?;

    Ok(())
}

// Selects all jobs with:
//  - expected_time in the past 
//  - error_message is null or executed_at is at least 60 minutes ago (intended to make repeat executions rare enough)
pub async fn get_jobs_to_execute(db: &DbClient) -> Result<Vec<Job>>  {
    let jobs = db
        .query(
            "
        SELECT * FROM jobs WHERE expected_time <= now() AND (error_message IS NULL OR executed_at <= now() - INTERVAL '60 minutes')",
            &[],
        )
        .await
        .context("Getting jobs data")?;

    let mut data = Vec::with_capacity(jobs.len());
    for job in jobs {
        let id: Uuid = job.get(0);
        let name: String = job.get(1);
        let job_type: JobType = job.get(2);
        let expected_time: DateTime<FixedOffset> = job.get(3);
        let cron_period: Option<i32> = job.get(4);
        let cron_unit: Option<CronUnit> = job.get(5);
        let metadata: serde_json::Value = job.get(6);
        let executed_at: Option<DateTime<FixedOffset>> = job.get(7);
        let error_message: Option<String> = job.get(8);

        data.push(Job {
            id,
            name,
            job_type,
            expected_time,
            cron_period,
            cron_unit,
            metadata,
            executed_at,
            error_message
        });
    }

    Ok(data)
}

pub fn get_duration_from_cron(cron_period: i32, cron_unit: &CronUnit) -> Duration {
    match cron_unit {
        CronUnit::Day => Duration::seconds(cron_period as i64) * DAY_IN_SECONDS,
        CronUnit::Hour => Duration::seconds(cron_period as i64) * HOUR_IN_SECONDS,
        CronUnit::Minute => Duration::seconds(cron_period as i64) * MINUTE_IN_SECONDS,
        CronUnit::Second => Duration::seconds(cron_period as i64),
    }
}
