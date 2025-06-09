mod commands;
mod event_handler;
use dotenv::dotenv;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let discord_token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set.");

    // Build our client.
    let mut discord_client = Client::builder(discord_token, GatewayIntents::empty())
        .event_handler(crate::event_handler::Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = discord_client.start().await {
        println!("Client error: {:?}", why);
    }
}
