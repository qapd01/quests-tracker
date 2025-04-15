use std::sync::Arc;
use anyhow::Result;
use axum::async_trait;

use crate::{domain::repositories::journey_ledger::JourneyLedgerRepository, infrastructure::postgres::postgres_connection::PgPoolSquad};

pub struct JournerLedgerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl JournerLedgerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl JourneyLedgerRepository for JournerLedgerPostgres {
    async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }
    async fn to_completed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }
    async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }
}