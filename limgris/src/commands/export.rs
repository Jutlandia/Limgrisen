use crate::lib::types::Ctf;
use serenity::builder::{CreateCommand, EditInteractionResponse, CreateMessage, CreateAttachment, CreateChannel, GetMessages};
use serenity::model::application::CommandInteraction;
use serenity::model::channel::{ChannelType, Message};
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::*;
use sqlx::SqlitePool;

use std::io::Write;
use chrono::{Utc, Datelike, DateTime};
use zip::ZipWriter;
use zip::write::FileOptions;
use std::collections::HashMap;

pub async fn run(
    pool: &SqlitePool,
    ctx: &Context,
    command: &CommandInteraction,
) -> String {
    // Defer the response immediately
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

    // Get CTF from current channel
    let ctf = match Ctf::fetch_by_snowflake(pool, &command.channel_id).await {
        Ok(ctf) => ctf,
        Err(_) => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content("Not a CTF channel. Please run this command from a CTF-related channel.")
            ).await;
            return "".to_string();
        }
    };

    // Check if CTF is archived - this command only works on archived CTFs
    if ctf.is_archived != Some(1) {
        let _ = command.edit_response(&ctx.http,
            EditInteractionResponse::new().content(&format!("CTF '{}' must be archived first! Use `/archive` command to archive it.", ctf.name))
        ).await;
        return "".to_string();
    }

    // Update status message
    let _ = command.edit_response(&ctx.http,
        EditInteractionResponse::new().content("🔄 Starting export process...")
    ).await;

    // Get all guild channels
    let guild_channels = match guild_id.channels(&ctx.http).await {
        Ok(channels) => channels,
        Err(e) => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content(&format!("Error getting guild channels: {}", e))
            ).await;
            return "".to_string();
        }
    };

    // Find all archived CTF-related channels (they should be in an "Archive - {ctf_name}" category)
    let archive_category_name = format!("Archive - {}", ctf.name);
    let mut archive_category_id = None;
    let mut ctf_channels = Vec::new();

    // First, find the archive category
    for (channel_id, channel) in guild_channels.iter() {
        if channel.kind == ChannelType::Category && channel.name == archive_category_name {
            archive_category_id = Some(*channel_id);
            break;
        }
    }

    let archive_category_id = match archive_category_id {
        Some(id) => id,
        None => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content(&format!("Could not find archive category '{}'. Make sure the CTF was properly archived.", archive_category_name))
            ).await;
            return "".to_string();
        }
    };

    // Find all channels in the archive category
    for (channel_id, channel) in guild_channels.iter() {
        if channel.parent_id == Some(archive_category_id) && channel.kind == ChannelType::Text {
            ctf_channels.push((*channel_id, channel.clone()));
        }
    }

    if ctf_channels.is_empty() {
        let _ = command.edit_response(&ctx.http,
            EditInteractionResponse::new().content(&format!("No channels found in archive category '{}'. Nothing to export.", archive_category_name))
        ).await;
        return "".to_string();
    }

    // Update status
    let _ = command.edit_response(&ctx.http,
        EditInteractionResponse::new().content(&format!("📥 Exporting chat history from {} archived channels...", ctf_channels.len()))
    ).await;

    // Create ZIP file with chat exports
    let zip_data = match create_ctf_export(&ctx, &ctf_channels, &ctf.name, command).await {
        Ok(data) => data,
        Err(e) => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content(&format!("Error creating export: {}", e))
            ).await;
            return "".to_string();
        }
    };

    // Get or create yearly reol channel
    let current_year = Utc::now().year();
    let reol_channel_name = format!("reol_{}", current_year);

    let reol_channel = match find_or_create_reol_channel(&ctx, guild_id, &reol_channel_name).await {
        Ok(channel) => channel,
        Err(e) => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content(&format!("Error accessing reol channel: {}", e))
            ).await;
            return "".to_string();
        }
    };

    // Update status
    let _ = command.edit_response(&ctx.http,
        EditInteractionResponse::new().content("📚 Uploading export to The Grand Archive...")
    ).await;

    // Create the ZIP attachment
    let zip_filename = format!("{}_export.zip", ctf.name.replace(" ", "_").replace("/", "_"));
    let zip_size_mb = zip_data.len() as f64 / (1024.0 * 1024.0); // Calculate size before moving
    let attachment = CreateAttachment::bytes(zip_data, &zip_filename);

    let archive_message = format!(
        "📚 **CTF Export: {}**\n\n\
        **Archive Details:**\n\
        • CTF Name: {}\n\
        • Channels Exported: {}\n\
        • Export Date: {}\n\
        • File Size: {:.2} MB",
        ctf.name, ctf.name, ctf_channels.len(),
        Utc::now().format("%Y-%m-%d %H:%M UTC"),
        zip_size_mb
    );

    match reol_channel.send_message(
        &ctx.http,
        CreateMessage::new()
            .content(&archive_message)
            .add_file(attachment)
    ).await {
        Ok(_) => {
            println!("Successfully uploaded CTF export for {}", ctf.name);
        }
        Err(e) => {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content(&format!("Error uploading export: {}", e))
            ).await;
            return "".to_string();
        }
    }

    // Note: Export completed successfully, could add is_exported field to database later if needed
    println!("Export completed for CTF: {}", ctf.name);

    // Get channel name for the final message
    let channel_name = match reol_channel.name(&ctx.http).await {
        Ok(name) => name,
        Err(_) => reol_channel_name.clone(),
    };

    // Final success message
    let result_message = format!(
        "✅ **Successfully exported CTF '{}'** to The Grand Archive\n\n\
        📚 Archive location: #{}\n\
        📋 Channels exported: {}\n\
        💾 File size: {:.2} MB\n\
        🗓️ Export year: {}\n\n\
        The knowledge is now preserved on the shelves of Jutlandia!",
        ctf.name,
        channel_name,
        ctf_channels.len(),
        zip_size_mb,
        current_year
    );

    let _ = command.edit_response(&ctx.http,
        EditInteractionResponse::new().content(&result_message)
    ).await;

    "".to_string()
}

/// Create a ZIP export of all chat history from CTF channels with attachments
async fn create_ctf_export(
    ctx: &Context,
    ctf_channels: &[(ChannelId, serenity::model::channel::GuildChannel)],
    ctf_name: &str,
    command: &CommandInteraction,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let mut zip_buffer = Vec::new();
    let mut downloaded_files: HashMap<String, Vec<u8>> = HashMap::new();
    let mut downloaded_avatars: HashMap<String, Vec<u8>> = HashMap::new();

    {
        let mut zip = ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // Create README file
        let readme_content = format!(
            "# CTF Export: {}\n\n\
            This archive contains chat exports from the {} CTF.\n\n\
            ## How to view:\n\
            1. Visit: https://github.com/slatinsky/DiscordChatExporter-frontend\n\
            2. Upload the JSON files from this archive\n\
            3. Browse the chat history in a user-friendly interface\n\n\
            ## Contents:\n\
            - README.md (this file)\n\
            - Individual JSON files for each channel\n\
            - assets/ folder with downloaded attachments\n\
            - avatars/ folder with user avatars\n\n\
            ## Channels included:\n",
            ctf_name, ctf_name
        );

        let mut full_readme = readme_content;
        for (_, channel) in ctf_channels {
            full_readme.push_str(&format!("- {}.json\n", channel.name));
        }

        full_readme.push_str(&format!(
            "\nGenerated on: {}\n\
            Total channels: {}\n\
            Velkommen til The Grand Archive of Jutlandia! 📚\n",
            Utc::now().format("%Y-%m-%d %H:%M UTC"),
            ctf_channels.len()
        ));

        zip.start_file("README.md", options)?;
        zip.write_all(full_readme.as_bytes())?;

        let total_channels = ctf_channels.len();
        let mut processed_channels = 0;

        // Export each channel
        for (channel_id, channel) in ctf_channels {
            processed_channels += 1;
            println!("Exporting channel: {} ({}/{})", channel.name, processed_channels, total_channels);

            // Update progress message
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content(&format!("📥 Exporting {} ({}/{}) and downloading attachments...", channel.name, processed_channels, total_channels))
            ).await;

            // Get messages from channel (limited to recent messages due to API limits)
            let messages = match channel_id.messages(&ctx.http, serenity::builder::GetMessages::new().limit(255)).await {
                Ok(msgs) => {
                    println!("Fetched {} messages from {}", msgs.len(), channel.name);
                    // Debug: Check first few messages for attachments
                    for (idx, msg) in msgs.iter().take(5).enumerate() {
                        println!("  Message {}: {} attachments, content length: {}", 
                                 idx, msg.attachments.len(), msg.content.len());
                        for att in &msg.attachments {
                            println!("    - Attachment: {} ({})", att.filename, att.url);
                        }
                    }
                    msgs
                },
                Err(e) => {
                    println!("Error getting messages from {}: {}", channel.name, e);
                    // Create empty channel file with error note
                    let error_content = format!(
                        "{{\"error\": \"Failed to export messages from {}: {}\", \"messageCount\": 0}}",
                        channel.name, e
                    );
                    let filename = format!("{}_ERROR.json", channel.name.replace("/", "_").replace("\\", "_"));
                    zip.start_file(&filename, options)?;
                    zip.write_all(error_content.as_bytes())?;
                    continue;
                }
            };

            // Download attachments and avatars from this channel's messages
            println!("Processing {} messages for attachments and avatars...", messages.len());
            for (msg_idx, message) in messages.iter().enumerate() {
                println!("Message {}: ID={}, attachments={}, content_len={}", 
                         msg_idx, message.id, message.attachments.len(), message.content.len());
                
                // Download user avatar
                if let Some(avatar_url) = message.author.avatar_url() {
                    let avatar_key = format!("{}.png", message.author.id);
                    
                    if !downloaded_avatars.contains_key(&avatar_key) {
                        println!("  Downloading avatar for: {} ({})", message.author.name, avatar_url);
                        
                        match download_attachment(&avatar_url).await {
                            Ok(data) => {
                                println!("    ✓ Downloaded avatar: {} bytes", data.len());
                                downloaded_avatars.insert(avatar_key, data);
                            }
                            Err(e) => {
                                println!("    ✗ Failed to download avatar for {}: {}", message.author.name, e);
                            }
                        }
                    }
                }

                // Download message attachments
                if !message.attachments.is_empty() {
                    println!("  Found {} attachments in this message", message.attachments.len());
                }
                for attachment in &message.attachments {
                    let file_key = format!("{}_{}", attachment.id, &attachment.filename);
                    println!("    Processing attachment: {} -> {}", attachment.filename, file_key);

                    if !downloaded_files.contains_key(&file_key) {
                        println!("      Downloading from: {}", attachment.url);

                        match download_attachment(&attachment.url).await {
                            Ok(data) => {
                                println!("      ✓ Downloaded: {} bytes", data.len());
                                downloaded_files.insert(file_key, data);
                            }
                            Err(e) => {
                                println!("      ✗ Failed to download {}: {}", attachment.filename, e);
                            }
                        }
                    } else {
                        println!("      Already downloaded");
                    }
                }
                
                // Download avatars for mentioned users
                for mentioned_user in &message.mentions {
                    if let Some(avatar_url) = mentioned_user.avatar_url() {
                        let avatar_key = format!("{}.png", mentioned_user.id);
                        
                        if !downloaded_avatars.contains_key(&avatar_key) {
                            println!("  Downloading avatar for mentioned user: {}", mentioned_user.name);
                            
                            match download_attachment(&avatar_url).await {
                                Ok(data) => {
                                    downloaded_avatars.insert(avatar_key, data);
                                }
                                Err(e) => {
                                    println!("    ✗ Failed to download avatar for {}: {}", mentioned_user.name, e);
                                }
                            }
                        }
                    }
                }
            }
            
            println!("Finished processing messages. Total attachments to save: {}", downloaded_files.len());

            // Convert to JSON format compatible with DiscordChatExporter
            let export_data = create_discord_chat_export(&messages, channel, ctf_name, &downloaded_files, &downloaded_avatars);
            let json_content = serde_json::to_string_pretty(&export_data)?;

            // Add to ZIP
            let filename = format!("{}.json", channel.name.replace("/", "_").replace("\\", "_"));
            zip.start_file(&filename, options)?;
            zip.write_all(json_content.as_bytes())?;
        }

        // Add all downloaded attachments to assets folder
        if !downloaded_files.is_empty() {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content("📎 Adding downloaded attachments to archive...")
            ).await;

            for (filename, data) in downloaded_files {
                let asset_path = format!("assets/{}", filename);
                zip.start_file(&asset_path, options)?;
                zip.write_all(&data)?;
            }
        }

        // Add all downloaded avatars to avatars folder
        if !downloaded_avatars.is_empty() {
            let _ = command.edit_response(&ctx.http,
                EditInteractionResponse::new().content("👤 Adding user avatars to archive...")
            ).await;

            for (filename, data) in downloaded_avatars {
                let avatar_path = format!("avatars/{}", filename);
                zip.start_file(&avatar_path, options)?;
                zip.write_all(&data)?;
            }
        }

        zip.finish()?;
    }

    Ok(zip_buffer)
}

/// Download an attachment from Discord CDN
async fn download_attachment(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

/// Create Discord Chat Export compatible JSON structure with local file paths
fn create_discord_chat_export(
    messages: &[Message],
    channel: &serenity::model::channel::GuildChannel,
    ctf_name: &str,
    downloaded_files: &HashMap<String, Vec<u8>>,
    downloaded_avatars: &HashMap<String, Vec<u8>>,
) -> serde_json::Value {
    let mut export_messages = Vec::new();
    for msg in messages.iter().rev() { // Reverse to get chronological order
        // Get local avatar path if available
        let avatar_key = format!("{}.png", msg.author.id);
        let author_avatar = if downloaded_avatars.contains_key(&avatar_key) {
            format!("avatars/{}", avatar_key)
        } else {
            msg.author.avatar_url().unwrap_or_default()
        };

        let message_data = serde_json::json!({
            "id": msg.id.to_string(),
            "type": "Default",
            "content": msg.content,
            "author": {
                "id": msg.author.id.to_string(),
                "name": msg.author.name,
                "discriminator": msg.author.discriminator.map(|d| d.to_string()).unwrap_or("0".to_string()),
                "nickname": msg.author.global_name.clone().unwrap_or(msg.author.name.clone()),
                "color": "#3498DB",
                "isBot": msg.author.bot,
                "avatarUrl": author_avatar
            },
            "timestamp": msg.timestamp.to_rfc3339(),
            "timestampEdited": msg.edited_timestamp.map(|t| t.to_rfc3339()),
            "callEndedTimestamp": serde_json::Value::Null,
            "isPinned": msg.pinned,
            "attachments": msg.attachments.iter().map(|att| {
                let file_key = format!("{}_{}", att.id, &att.filename);
                let local_url = if downloaded_files.contains_key(&file_key) {
                    format!("assets/{}", file_key)
                } else {
                    att.url.clone() // Fallback to original URL if download failed
                };

                serde_json::json!({
                    "id": att.id.to_string(),
                    "url": local_url,
                    "fileName": att.filename,
                    "fileSizeBytes": att.size
                })
            }).collect::<Vec<_>>(),
            "embeds": msg.embeds.iter().map(|embed| serde_json::json!({
                "title": embed.title,
                "description": embed.description,
                "url": embed.url,
                "color": embed.colour.map(|c| c.0),
                "timestamp": embed.timestamp.map(|t| t.to_rfc3339()),
                "footer": embed.footer.as_ref().map(|f| serde_json::json!({
                    "text": f.text,
                    "iconUrl": f.icon_url
                })),
                "image": embed.image.as_ref().map(|img| serde_json::json!({
                    "url": img.url,
                    "width": img.width,
                    "height": img.height
                })),
                "thumbnail": embed.thumbnail.as_ref().map(|thumb| serde_json::json!({
                    "url": thumb.url,
                    "width": thumb.width,
                    "height": thumb.height
                })),
                "author": embed.author.as_ref().map(|author| serde_json::json!({
                    "name": author.name,
                    "url": author.url,
                    "iconUrl": author.icon_url
                })),
                "fields": embed.fields.iter().map(|field| serde_json::json!({
                    "name": field.name,
                    "value": field.value,
                    "inline": field.inline
                })).collect::<Vec<_>>()
            })).collect::<Vec<_>>(),
            "stickers": [],
            "reactions": [],
            "mentions": msg.mentions.iter().map(|user| {
                // Get local avatar path for mentioned user if available
                let mention_avatar_key = format!("{}.png", user.id);
                let mention_avatar = if downloaded_avatars.contains_key(&mention_avatar_key) {
                    format!("avatars/{}", mention_avatar_key)
                } else {
                    user.avatar_url().unwrap_or_default()
                };

                serde_json::json!({
                    "id": user.id.to_string(),
                    "name": user.name,
                    "discriminator": user.discriminator.map(|d| d.to_string()).unwrap_or("0".to_string()),
                    "nickname": user.global_name.clone().unwrap_or(user.name.clone()),
                    "color": "#3498DB",
                    "isBot": user.bot,
                    "avatarUrl": mention_avatar
                })
            }).collect::<Vec<_>>()
        });
        export_messages.push(message_data);
    }

    serde_json::json!({
        "guild": {
            "id": channel.guild_id.to_string(),
            "name": format!("Jutlandia CTF: {}", ctf_name),
            "iconUrl": ""
        },
        "channel": {
            "id": channel.id.to_string(),
            "type": match channel.kind {
                ChannelType::Text => "GuildTextChat",
                ChannelType::Voice => "GuildVoiceChat",
                _ => "GuildTextChat"
            },
            "categoryId": channel.parent_id.map(|id| id.to_string()).unwrap_or_else(|| "0".to_string()),
            "category": "CTF Challenges",
            "name": channel.name,
            "topic": channel.topic.clone()
        },
        "dateRange": {
            "after": serde_json::Value::Null,
            "before": serde_json::Value::Null
        },
        "exportedAt": Utc::now().to_rfc3339(),
        "messages": export_messages,
        "messageCount": export_messages.len()
    })
}

/// Find existing reol channel or create new one
async fn find_or_create_reol_channel(
    ctx: &Context,
    guild_id: GuildId,
    channel_name: &str,
) -> Result<ChannelId, Box<dyn std::error::Error + Send + Sync>> {
    let channels = guild_id.channels(&ctx.http).await?;

    // Look for existing channel
    for (channel_id, channel) in channels.iter() {
        if channel.kind == ChannelType::Text && channel.name == channel_name {
            return Ok(*channel_id);
        }
    }

    // Create new channel
    let channel = guild_id
        .create_channel(
            &ctx.http,
            CreateChannel::new(channel_name).kind(ChannelType::Text),
        )
        .await?;

    Ok(channel.id)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("export")
        .description("Export archived CTF chat history to The Grand Archive (reol system)")
}
