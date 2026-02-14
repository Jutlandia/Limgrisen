use crate::lib::types::Ctf;
use serenity::builder::{CreateCommand, CreateChannel, EditChannel,EditInteractionResponse};
use serenity::model::application::CommandInteraction;
use serenity::model::channel::ChannelType;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::*;
use serenity::all::Permissions;
use sqlx::SqlitePool;
use std::str::FromStr;

pub async fn run(
    pool: &SqlitePool,
    ctx: &Context,
    command: &CommandInteraction,
) -> String {
    
    if let Err(why) = command.defer(&ctx.http).await {
          return format!("Failed to defer response: {}", why);
    }

    let guild_id = match command.guild_id {
        Some(id) => id,
        None => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content("This command can only be used in a server!")
            ).await;
            return "".to_string();
        }
    };

    // Check if user has administrator permissions
    if let Some(member) = &command.member {
        let permissions = member.permissions.unwrap_or(Permissions::empty());
        if !permissions.contains(Permissions::ADMINISTRATOR) {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content("❌ You need Administrator permissions to archive CTFs!")
            ).await;
            return "".to_string();
        }
    } else {
        let _ = command.edit_response(&ctx.http,
            EditInteractionResponse::new().content("❌ Could not verify your permissions!")
        ).await;
        return "".to_string();
    }

    // Use the same pattern as challenge command - fetch CTF by channel snowflake
    let ctf = match Ctf::fetch_by_snowflake(pool, &command.channel_id).await {
        Ok(ctf) => ctf,
        Err(_) => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content("Not a CTF channel. Please run this command from a CTF-related channel.")
            ).await;
            return "".to_string();
        }
    };

    // Check if already archived (using field, not method)
    if ctf.is_archived == Some(1) {
        let _ = command.edit_response(&ctx.http,
            EditInteractionResponse::new().content(&format!("CTF '{}' is already archived!", ctf.name))
        ).await;
        return "".to_string();
    }

    // Get all guild channels
    let guild_channels = match guild_id.channels(&ctx.http).await {
        Ok(channels) => channels,
        Err(e) => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content(&format!("Error getting guild channels: {}", e))
            ).await;
            return "".to_string()
        }
    };

    // Create archive category
    let archive_category_name = format!("Archive - {}", ctf.name);
    let archive_category = match guild_id
        .create_channel(
            &ctx.http,
            CreateChannel::new(&archive_category_name).kind(ChannelType::Category),
        )
        .await
    {
        Ok(category) => category,
        Err(e) => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content(&format!("Error creating archive category: {}", e))
            ).await;
            return "".to_string()
        }
    };

    // Find and move all related channels
    let mut moved_channels = 0;
    for (channel_id, channel) in guild_channels.iter() {
        // Check if channel belongs to this CTF
        if is_ctf_channel(&channel.name, &ctf.name) {
            // Move channel to archive category
            let editor = EditChannel::new().category(archive_category.id);
            match channel_id.edit(&ctx.http, editor).await {
                Ok(_) => {
                    moved_channels += 1;
                    println!("Moved channel '{}' to archive", channel.name);
                }
                Err(e) => {
                    println!("Error moving channel '{}': {}", channel.name, e);
                }
            }
        }
    }

    // Mark CTF as archived in database
    match sqlx::query!(
        "UPDATE ctfs SET is_archived = 1 WHERE id = ?",
        ctf.id // Using field directly
    )
    .execute(pool)
    .await
    {
        Ok(_) => {}
        Err(e) => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content(&format!("Error updating database: {}", e))
            ).await;
            return "".to_string()
        }
    }

    // Also mark all challenges as archived
    match sqlx::query!(
        "UPDATE challenges SET is_archived = 1 WHERE ctf_id = ?",
        ctf.id // Using field directly
    )
    .execute(pool)
    .await
    {
        Ok(_) => {}
        Err(e) => {
            println!("Warning: Error archiving challenges: {}", e);
        }
    }
    let result_message = format!(
        "✅ Successfully archived CTF '{}'\n📁 Created category: {}\n📋 Moved {} channels to archive",
        ctf.name, archive_category_name, moved_channels
    );

    let _ = command.edit_response(&ctx.http,
        EditInteractionResponse::new().content(&result_message)
    ).await;

    "".to_string()
}

/// Check if a channel belongs to the specified CTF
fn is_ctf_channel(channel_name: &str, ctf_name: &str) -> bool {
    // Exact match (main CTF channel)
    if channel_name == ctf_name {
        return true;
    }

    // Challenge channel pattern: starts with ctf_name followed by hyphen
    if channel_name.starts_with(&format!("{}-", ctf_name)) {
        return true;
    }

    false
}

pub fn register() -> CreateCommand {
    CreateCommand::new("archive")
        .description("Archive the current CTF and move all related channels to archive category")
}
