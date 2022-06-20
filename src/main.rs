use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: String,
) -> Result<(), Error> {
    // let u = user.as_ref().unwrap_or(ctx.author());
    // let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.send(|f| f
        .content(user)
        .embed(|f| f
            .title("Much versatile, very wow")
            .description("I need more documentation ok?")
        )
        .ephemeral(true) // this one only applies in application commands though
    ).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: vec![age(), register()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}