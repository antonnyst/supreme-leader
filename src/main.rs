use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

const VIOLATION_MESSAGE: &str = 
    "VERY BAD! 20 social credits have been deducted 低等公民 and your internet access card 上网通行证 has been suspended for: [24 Hours]. Please refrain from mentioning events that never happened that could discredit the great 人民共产党 People’s Communist Party again or we will be forced to 饿了就睡觉 send party agents to escort you to a re-education van [人民行刑车].";

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name == "supreme leader" {
            return;
        }
        println!("Message sent in channel {:?}: {:?}", 
            msg.channel_id.to_string(), 
            msg.content.to_string()
        );
    
        let channel_name = match msg.channel(&ctx).await {
            Ok(channel) => {
                match channel.guild() {
                    Some(guild_channel) => {
                        guild_channel.name().to_string()
                    },
                    None => {
                        println!("Not a guild channel?!");
                        "".to_string()
                    },
                }
            },
            Err(why) => {
                println!("Error getting channel: {:?}", why);
                "".to_string()
            },  
        };

        if channel_name.eq("tiananmen-square") {
            if let Err(why) = msg.channel_id.delete_message(&ctx.http, msg.id).await {
                println!("Error deleting message: {:?}", why);
            }
           
            if let Err(why) = msg.author.direct_message(&ctx.http, |m| m.content(VIOLATION_MESSAGE)).await {
                println!("Error sending message: {:?}", why);
            }

        }

    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
