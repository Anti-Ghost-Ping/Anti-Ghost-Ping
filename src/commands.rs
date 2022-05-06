use std::sync::Arc;

use anyhow::Result;
use twilight_model::{
    application::{command::{
        BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, Command,
        CommandOption, CommandType
    }, interaction::application_command::CommandData},
    channel::ChannelType,
    id::Id,
};

use crate::structs::AgpContext;

#[allow(dead_code)]
pub fn commands() -> [Command; 5] {
    [
        Command {
            application_id: None,
            default_permission: None,
            description: "Set a channel for all ghost ping messages to get redirected to"
                .to_string(),
            guild_id: None,
            id: None,
            kind: CommandType::ChatInput,
            name: "redirect".to_string(),
            options: vec![CommandOption::Channel(ChannelCommandOptionData {
                channel_types: vec![ChannelType::GuildText, ChannelType::GuildNews],
                description: "The channel for alerts to redirect to".to_string(),
                name: "channel".to_string(),
                required: true,
            })],
            version: Id::new(1),
        },
        Command {
            application_id: None,
            default_permission: None,
            description: "Toggle detection for `@everyone` and `@here` ghost pings".to_string(),
            guild_id: None,
            id: None,
            kind: CommandType::ChatInput,
            name: "etoggle".to_string(),
            options: vec![CommandOption::Boolean(BaseCommandOptionData {
                description: "The option to enable or disable this setting".to_string(),
                name: "enable".to_string(),
                required: true,
            })],
            version: Id::new(1),
        },
        Command {
            application_id: None,
            default_permission: None,
            description: "Toggle for alerts to only contain mentions and no message content"
                .to_string(),
            guild_id: None,
            id: None,
            kind: CommandType::ChatInput,
            name: "mentiononly".to_string(),
            options: vec![CommandOption::Boolean(BaseCommandOptionData {
                description: "The option to enable or disable this setting".to_string(),
                name: "enable".to_string(),
                required: true,
            })],
            version: Id::new(1),
        },
        Command {
            application_id: None,
            default_permission: None,
            description: "Set a color for the ghost ping alert".to_string(),
            guild_id: None,
            id: None,
            kind: CommandType::ChatInput,
            name: "setcolor".to_string(),
            options: vec![CommandOption::String(ChoiceCommandOptionData {
                autocomplete: true,
                choices: vec![],
                description: "Color to set the alert".to_string(),
                name: "color".to_string(),
                required: true,
            })],
            version: Id::new(1),
        },
        Command {
            application_id: None,
            default_permission: None,
            description: "Info about Anti Ghost Ping".to_string(),
            guild_id: None,
            id: None,
            kind: CommandType::ChatInput,
            name: "info".to_string(),
            options: vec![],
            version: Id::new(1),
        },
    ]
}

pub async fn redirect(ctx: Arc<AgpContext>, data: CommandData) -> Result<()> {
    Ok(())
}

pub async fn etoggle(ctx: Arc<AgpContext>, data: CommandData) -> Result<()> {
    Ok(())
}

pub async fn mentiononly(ctx: Arc<AgpContext>, data: CommandData) -> Result<()> {
    Ok(())
}

pub async fn setcolor(ctx: Arc<AgpContext>, data: CommandData) -> Result<()> {
    Ok(())
}

pub async fn info(ctx: Arc<AgpContext>) -> Result<()> {
    Ok(())
}
