use twilight_model::{
    application::command::{
        BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, Command,
        CommandOption, CommandType,
    },
    channel::ChannelType,
    guild::Permissions,
    id::Id,
};

use crate::structs::AgpContext;

pub mod color;
pub mod database;
pub mod embed;
pub mod macros;
pub mod message;

impl AgpContext {
    pub fn interaction(&self) -> twilight_http::client::InteractionClient {
        self.http.interaction(self.app_id)
    }
}

pub fn commands() -> [Command; 5] {
    [
        Command {
            application_id: None,
            description: "Set a channel for all ghost ping messages to get redirected to"
                .to_string(),
            guild_id: None,
            id: None,
            kind: CommandType::ChatInput,
            name: "redirect".to_string(),
            options: vec![CommandOption::Channel(ChannelCommandOptionData {
                channel_types: vec![ChannelType::GuildText, ChannelType::GuildNews],
                description:
                    "The channel for alerts to redirect to (Don't set to reset this option)"
                        .to_string(),
                name: "channel".to_string(),
                required: false,
                description_localizations: None,
                name_localizations: None,
            })],
            version: Id::new(1),
            default_member_permissions: Some(Permissions::ADMINISTRATOR),
            dm_permission: Some(false),
            description_localizations: None,
            name_localizations: None,
        },
        Command {
            application_id: None,
            description: "Toggle detection for `@everyone` and `@here` ghost pings".to_string(),
            guild_id: None,
            id: None,
            kind: CommandType::ChatInput,
            name: "etoggle".to_string(),
            options: vec![CommandOption::Boolean(BaseCommandOptionData {
                description: "The option to enable or disable this setting".to_string(),
                name: "enable".to_string(),
                required: true,
                description_localizations: None,
                name_localizations: None,
            })],
            version: Id::new(1),
            default_member_permissions: Some(Permissions::ADMINISTRATOR),
            dm_permission: Some(false),
            description_localizations: None,
            name_localizations: None,
        },
        Command {
            application_id: None,
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
                description_localizations: None,
                name_localizations: None,
            })],
            version: Id::new(1),
            default_member_permissions: Some(Permissions::ADMINISTRATOR),
            dm_permission: Some(false),
            description_localizations: None,
            name_localizations: None,
        },
        Command {
            application_id: None,
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
                description_localizations: None,
                name_localizations: None,
            })],
            version: Id::new(1),
            default_member_permissions: Some(Permissions::ADMINISTRATOR),
            dm_permission: Some(false),
            description_localizations: None,
            name_localizations: None,
        },
        Command {
            application_id: None,
            description: "Info about Anti Ghost Ping".to_string(),
            guild_id: None,
            id: None,
            kind: CommandType::ChatInput,
            name: "info".to_string(),
            options: vec![],
            version: Id::new(1),
            default_member_permissions: None,
            dm_permission: Some(true),
            description_localizations: None,
            name_localizations: None,
        },
    ]
}
