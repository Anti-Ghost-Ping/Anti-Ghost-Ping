#[derive(Debug)]
pub struct GuildConfig {
    pub guild_id: i64,
    // Planned for removal
    pub prefix: Option<String>,
    pub channel_id: Option<i64>,
    pub everyone: bool,
    pub mention_only: bool,
    pub color: Option<i32>,
}
