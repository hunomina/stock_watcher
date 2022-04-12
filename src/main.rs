#[macro_use]
extern crate dotenv_codegen;

mod commands;
mod storage;

use std::collections::HashMap;

use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use serenity::async_trait;
use serenity::client::{Client as SerenityClient, Context, EventHandler};
use serenity::framework::standard::{macros::command, CommandResult, StandardFramework};
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::model::{
    channel::Message,
    guild::{GuildStatus, PartialGuild},
    prelude::Ready,
};

use storage::subscriber::{Subscriber, SubscriberRepository};

struct Bot {
    db_config: ClientOptions,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        //println!("message: {:?}", msg);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                commands::init::COMMAND_NAME => self.init(&ctx, &command).await,
                commands::list::COMMAND_NAME => self.list(&ctx, &command).await,
                commands::watch::COMMAND_NAME => self.watch(&ctx, &command).await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(e) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Error responding to slash command: {}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready
            .guilds
            .iter()
            .for_each(|guild| println!("{:?}", guild));
        for guild in ready.guilds.into_iter() {
            let _ = guild
                .id()
                .set_application_commands(ctx.http.clone(), commands::get_commands)
                .await;
        }
    }
}

impl Bot {
    async fn list(&self, ctx: &Context, interaction: &ApplicationCommandInteraction) -> String {
        let repository = SubscriberRepository {
            database_connection: self.get_dabatase_connection(),
        };
        let subscribers = repository.all().await;

        let subscribers = match subscribers {
            Ok(subscribers) => subscribers,
            Err(e) => {
                println!("{}", e);
                vec![]
            }
        };

        match subscribers.len() {
            0 => "No subscribers".to_string(),
            _ => subscribers
                .into_iter()
                .fold(String::new(), |mut acc, subscriber| {
                    acc.push_str(&format!(
                        "{}: {:?}\n",
                        subscriber.id, subscriber.subscriptions
                    ));
                    acc
                }),
        }
    }

    async fn init(&self, ctx: &Context, interaction: &ApplicationCommandInteraction) -> String {
        let repository = SubscriberRepository {
            database_connection: self.get_dabatase_connection(),
        };

        let search = repository.get(interaction.channel_id.clone().0).await;

        if let Err(e) = search {
            println!("{:?}", e);
            return "An error occured while checking if this channel is already subscribed.".into();
        }

        let search = search.unwrap();

        if let Some(_) = search {
            return "This channel is already subscribed.".into();
        }

        match repository
            .add(Subscriber::new(interaction.channel_id))
            .await
        {
            Ok(_) => "This channel has been added to the list of subscribers.",
            Err(e) => {
                println!("{}", e);
                "An error occured while adding this channel to the subscriber list."
            }
        }
        .into()
    }

    async fn watch(&self, ctx: &Context, interaction: &ApplicationCommandInteraction) -> String {
        let symbol_option = interaction.data.options.get(0);

        if let None = symbol_option {
            return "Symbol parameter missing.".into();
        }

        let symbol_option = symbol_option.unwrap().resolved.as_ref().unwrap();

        if let ApplicationCommandInteractionDataOptionValue::String(symbol) = symbol_option {
            format!("Symbol \"{}\" is now watched", symbol)
        } else {
            "Something wrong happened".into()
        }

        // todo : check if symbol exist

        //let repository = SubscriberRepository {
        //    database_connection: self.get_dabatase_connection(),
        //};

        //let search = repository.get(interaction.channel_id.clone().0).await;

        //if let Err(e) = search {
        //    println!("{:?}", e);
        //    return "An error occured while checking if this channel is already subscribed.".into();
        //}

        //let search = search.unwrap();

        //if let None = search {
        //    return "This channel is not a subscriber.".into();
        //}

        //let mut subscriber = search.unwrap();

        //subscriber.subscriptions.push(value)
    }

    fn get_dabatase_connection(&self) -> Database {
        Client::with_options(self.db_config.clone())
            .unwrap()
            .database(dotenv!("MONGO_INITDB_DATABASE"))
    }

    //fn map_command_error(error: CommandError) -> String {
    //    println!("{}", error);
    //    String::from("An error occured while running the command")
    //}
}

#[tokio::main]
async fn main() {
    // start listening for events by starting a single shard
    if let Err(why) = get_discord_client().await.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

async fn get_database_config() -> ClientOptions {
    ClientOptions::parse(format!(
        "mongodb://{}:{}@{}",
        dotenv!("MONGO_INITDB_ROOT_USERNAME"),
        dotenv!("MONGO_INITDB_ROOT_PASSWORD"),
        dotenv!("MONGO_DB_HOST"),
    ))
    .await
    .unwrap()
}

async fn get_discord_client() -> SerenityClient {
    let framework = StandardFramework::new().configure(|c| c.allow_dm(false));

    let bot = Bot {
        db_config: get_database_config().await,
    };

    SerenityClient::builder(dotenv!("DISCORD_TOKEN"))
        .event_handler(bot)
        .framework(framework)
        .application_id(dotenv!("DISCORD_APP_ID").parse().unwrap())
        .await
        .expect("Error creating client")
}

// todo : (un)watch {stock name} : add stock watcher to server; do nothing if channel not init; send confirm message on ok + check symbol exists
