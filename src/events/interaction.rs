use std::sync::Arc;

use anyhow::Result;
use tracing::{info, warn};
use twilight_model::{
    application::interaction::{
        application_command::CommandOptionValue, ApplicationCommandAutocomplete, Interaction,
    },
    http::interaction::InteractionResponse,
};

use crate::{commands::*, structs::AgpContext};

pub async fn handle_interaction(ctx: Arc<AgpContext>, interaction: Interaction) -> Result<()> {
    let command = match interaction {
        Interaction::ApplicationCommand(cmd) => cmd,
        Interaction::ApplicationCommandAutocomplete(ac) => {
            return handle_auto_complete(ac).await;
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
                    Some(chn.get())
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
        "etoggle" => etoggle::etoggle(ctx.clone(), command.data).await?,
        "mentiononly" => mentiononly::mentiononly(ctx.clone(), command.data).await?,
        "setcolor" => setcolor::setcolor(ctx.clone(), command.data).await?,
        "info" => info::info(ctx.clone()).await?,
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

pub async fn handle_auto_complete(ac: Box<ApplicationCommandAutocomplete>) -> Result<()> {
    info!("{:#?}", ac);
    Ok(())
}
