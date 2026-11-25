use crate::state::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Current version of the bot is 1.0.0!").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "First number"] a: i64,
    #[description = "Second number"] b: i64,
) -> Result<(), Error> {
    let result = a + b;
    ctx.say(format!("{} + {} = **{}**", a, b, result)).await?;
    Ok(())
}
