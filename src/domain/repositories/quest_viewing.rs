use crate::domain::{
    entities::quests::QuestEntity, value_objects::board_checking_filter::BoardCheckingFilter,
};

use anyhow::Result;
use axum::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait QuestViewingRepository {
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity>;
    async fn borad_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>>;
    async fn adventurers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64>;
}
