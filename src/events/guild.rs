use std::sync::Arc;

use twilight_model::gateway::payload::incoming::GuildDelete;
use anyhow::Result;

use crate::context::AgpContext;

pub async fn on_guild_leave(
    ctx: Arc<AgpContext>,
    guild: &GuildDelete,
) -> Result<()> {
    sqlx::query!(
        "DELETE FROM guild_configs WHERE guild_id = $1",
        guild.id.get() as i64
    )
    .execute(&ctx.db)
    .await?;
    Ok(())
}
