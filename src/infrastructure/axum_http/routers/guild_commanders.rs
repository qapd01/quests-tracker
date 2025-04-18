use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};

use crate::{
    application::usecases::guild_commanders::GuildCommandersUseCase,
    domain::{
        repositories::guild_commanders::GuildCommandersRepository,
        value_objects::guild_commander_model::RegisterGuildCommanderModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::guild_commanders::GuildCommanderPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    // repository
    let guild_commanders_repository = GuildCommanderPostgres::new(db_pool);
    // usecase
    let guild_commanders_use_case =
        GuildCommandersUseCase::new(Arc::new(guild_commanders_repository));
    // router state
    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commanders_use_case))
}

pub async fn register<T>(
    State(guild_commanders_use_case): State<Arc<GuildCommandersUseCase<T>>>,
    Json(register_guild_commander_model): Json<RegisterGuildCommanderModel>,
) -> impl IntoResponse
where
    T: GuildCommandersRepository + Send + Sync,
{
    // unimplemented!()
}
