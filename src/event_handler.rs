use serenity::{
    async_trait,
    model::{
        application::interaction::{Interaction, InteractionResponseType},
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};
use crate::commands;

async fn not_impl() -> String {
    return "Not implemented".to_string();
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(mut command) = interaction {
            let content = match command.data.name.as_str() {
                "get_user_data" => commands::get_user_data::get_user_data(&mut command).await,
                _ => not_impl().await,
            };

            if content.len() > 2000 {
                use std::fs;
                let _ = fs::create_dir("./tmp");

                fs::write("./tmp/message.txt", &content).expect("Unable to write file");

                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.add_file("./tmp/message.txt")
                            })
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
                let _ = fs::remove_file("./tmp/message.txt");
            } else {
                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(content))
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            std::env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _ = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::get_user_data::register(command))
        })
        .await;
    }
}
