use std::sync::Arc;

use anyhow::Result;
use tracing::{warn, info};
use twilight_model::application::interaction::{Interaction, ApplicationCommandAutocomplete};

use crate::{commands, structs::AgpContext};

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

    match command.data.name.as_str() {
        "redirect" => {
            commands::redirect(ctx, command.data).await?;
        }
        "etoggle" => {
            commands::etoggle(ctx, command.data).await?;
        }
        "mentiononly" => {
            commands::mentiononly(ctx, command.data).await?;
        }
        "setcolor" => {
            commands::setcolor(ctx, command.data).await?;
        }
        "info" => {
            commands::info(ctx).await?;
        }
        _ => {
            warn!("Unhandled command: {:#?}", command);
        }
    }

    Ok(())
}

pub async fn handle_auto_complete(ac: Box<ApplicationCommandAutocomplete>) -> Result<()> {
    info!("{:#?}", ac);
    Ok(())
}
