use std::sync::Arc;

use anyhow::Result;
use tracing::warn;
use twilight_model::application::interaction::Interaction;

use crate::{commands, context::AgpContext};

pub async fn handle_interaction(ctx: Arc<AgpContext>, interaction: Interaction) -> Result<()> {
    let command = match interaction {
        Interaction::ApplicationCommand(cmd) => cmd,
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
        _ => {
            warn!("Unhandled command: {:#?}", command);
        }
    }

    Ok(())
}
