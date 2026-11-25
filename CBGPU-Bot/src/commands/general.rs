use crate::state::{Context, Error};

#[poise::command(slash_command, prefix_command, rename = "cbgpu_info")]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("Current version of the bot is {}!", env!("CARGO_PKG_VERSION"))).await?;
    Ok(())
}
