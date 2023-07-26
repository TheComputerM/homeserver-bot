use dotenv::dotenv;
use poise::serenity_prelude as serenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn system(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("TODO").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![system()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
