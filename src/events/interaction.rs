use std::sync::Arc;

use anyhow::Result;
use tracing::warn;
use twilight_model::{
    application::{
        command::CommandOptionChoice,
        interaction::{
            application_command::CommandOptionValue, ApplicationCommandAutocomplete, Interaction,
        },
    },
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

use crate::{commands::*, helpers::color::parse_color, structs::AgpContext};

pub async fn handle_interaction(ctx: Arc<AgpContext>, interaction: Interaction) -> Result<()> {
    let command = match interaction {
        Interaction::ApplicationCommand(cmd) => cmd,
        Interaction::ApplicationCommandAutocomplete(ac) => {
            return handle_auto_complete(ctx.clone(), ac).await;
        }
        unknown => {
            warn!("Unhandled interaction: {:#?}", unknown);
            return Ok(());
        }
    };

    let resp: InteractionResponse = match command.data.name.as_str() {
        "redirect" => {
            let channel = if let Some(opt) = &command.data.options.get(0) {
                if let CommandOptionValue::Channel(chn) = opt.value {
                    Some(chn.get() as i64)
                } else {
                    None
                }
            } else {
                None
            };

            let guild_id = if let Some(id) = command.guild_id {
                id.get() as i64
            } else {
                return Ok(());
            };

            redirect::redirect(ctx.clone(), channel, guild_id).await?
        }
        "etoggle" => {
            let choice = if let Some(opt) = command.data.options.get(0) {
                if let CommandOptionValue::Boolean(bool) = opt.value {
                    bool
                } else {
                    false
                }
            } else {
                false
            };

            let guild_id = if let Some(id) = command.guild_id {
                id.get() as i64
            } else {
                return Ok(());
            };

            etoggle::etoggle(ctx.clone(), choice, guild_id).await?
        }
        "mentiononly" => {
            let choice = if let Some(opt) = command.data.options.get(0) {
                if let CommandOptionValue::Boolean(bool) = opt.value {
                    bool
                } else {
                    false
                }
            } else {
                false
            };

            let guild_id = if let Some(id) = command.guild_id {
                id.get() as i64
            } else {
                return Ok(());
            };

            mentiononly::mentiononly(ctx.clone(), choice, guild_id).await?
        }
        "setcolor" => {
            let color = if let Some(opt) = command.data.options.get(0) {
                if let CommandOptionValue::String(input) = &opt.value {
                    parse_color(input.to_string())
                } else {
                    Ok(16711712)
                }
            } else {
                Ok(16711712)
            };

            let guild_id = if let Some(id) = command.guild_id {
                id.get() as i64
            } else {
                return Ok(());
            };
            setcolor::setcolor(ctx.clone(), color, guild_id).await?
        }
        _ => {
            warn!("Unhandled command: {:#?}", command);
            return Ok(());
        }
    };

    ctx.interaction()
        .create_response(command.id, &command.token, &resp)
        .exec()
        .await?;

    Ok(())
}

pub async fn handle_auto_complete(
    ctx: Arc<AgpContext>,
    ac: Box<ApplicationCommandAutocomplete>,
) -> Result<()> {
    if ac.data.name == "setcolor" {
        let resp = InteractionResponse {
            kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
            data: Some(InteractionResponseData {
                choices: Some(vec![
                    CommandOptionChoice::String {
                        name: "random".to_string(),
                        name_localizations: None,
                        value: "random".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "teal".to_string(),
                        name_localizations: None,
                        value: "teal".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "green".to_string(),
                        name_localizations: None,
                        value: "green".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "blue".to_string(),
                        name_localizations: None,
                        value: "blue".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "purple".to_string(),
                        name_localizations: None,
                        value: "purple".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "magenta".to_string(),
                        name_localizations: None,
                        value: "magenta".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "gold".to_string(),
                        name_localizations: None,
                        value: "gold".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "orange".to_string(),
                        name_localizations: None,
                        value: "orange".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "red".to_string(),
                        name_localizations: None,
                        value: "red".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "yellow".to_string(),
                        name_localizations: None,
                        value: "yellow".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "og blurple".to_string(),
                        name_localizations: None,
                        value: "og blurple".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "blurple".to_string(),
                        name_localizations: None,
                        value: "blurple".to_string(),
                    },
                    CommandOptionChoice::String {
                        name: "dark theme".to_string(),
                        name_localizations: None,
                        value: "dark theme".to_string(),
                    },
                ]),
                allowed_mentions: None,
                attachments: None,
                components: None,
                content: None,
                custom_id: None,
                embeds: None,
                flags: None,
                title: None,
                tts: None,
            }),
        };

        ctx.interaction()
            .create_response(ac.id, &ac.token, &resp)
            .exec()
            .await?;
    }
    Ok(())
}
