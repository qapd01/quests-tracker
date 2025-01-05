use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::postgres::schema::guild_commanders;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = guild_commanders)]
pub struct GuildCommanderEntitiy {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = guild_commanders)]
pub struct RegisterGuildCommanderEntitiy {
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}